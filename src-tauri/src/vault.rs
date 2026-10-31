//! Vault file format and session handling.
//!
//! A random data key (DK) encrypts the entries. The DK is stored twice:
//! wrapped by the master password, and optionally wrapped by the recovery answer.
//! Changing the master password only rewraps the DK; entries are not re-encrypted.
//!
//! Stored files are separate blobs in `files/`, sealed with the DK. Their index
//! (name, size, hash, linked login) is sealed inside `vault.json`.
//!
//! A backup bundles the master wrap, the sealed data and the raw blobs:
//! `CRYPTBAK | u32 LE header length | header JSON | blobs`.
//! It opens with the master password that was current at export time.

use std::{
    collections::{HashMap, HashSet},
    fmt,
    fs::{self, File},
    io::{self, BufWriter, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard, PoisonError},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::crypto::{self, from_hex, to_hex, CryptoError, KeyBytes, Sealed, SALT_LEN};

const FORMAT_VERSION: u32 = 3;
const FILE: &str = "vault.json";
const FILES_DIR: &str = "files";
const LEGACY_FILE: &str = "passwords.bin";
const BACKUP_FILE: &str = "passwords.bin.bak";
const BACKUP_MAGIC: &[u8; 8] = b"CRYPTBAK";
/// Backup headers hold sealed logins and the file index. Anything bigger is not ours.
const MAX_HEADER_BYTES: u32 = 64 << 20;
pub const MAX_FILE_BYTES: u64 = 100 << 20;
/// Nonce and tag added by `crypto::seal_raw`.
const BLOB_OVERHEAD: u64 = 28;
const MIN_PASSWORD_LEN: usize = 8;
const MIN_ANSWER_LEN: usize = 3;

const AAD_MASTER: &[u8] = b"crypt/v2/master";
const AAD_RECOVERY: &[u8] = b"crypt/v2/recovery";
const AAD_DATA: &[u8] = b"crypt/v2/data";
const AAD_FILES: &[u8] = b"crypt/v2/files";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Entry {
    pub id: u64,
    pub site: String,
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub notes: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

/// A stored file. Contents live in `files/<id>.bin`.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileMeta {
    /// Below 2^53 so it survives JavaScript numbers.
    pub id: u64,
    pub name: String,
    pub size: u64,
    pub sha256: String,
    /// Unix milliseconds.
    pub added: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_id: Option<u64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub exists: bool,
    pub recovery_question: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportSummary {
    pub entries: Vec<Entry>,
    pub files: Vec<FileMeta>,
    pub added_entries: usize,
    pub added_files: usize,
    pub skipped: usize,
}

#[derive(Serialize, Deserialize)]
struct Wrap {
    salt: String,
    key: Sealed,
}

impl Wrap {
    /// Wrap `dk` under a key derived from `secret`, with a fresh salt.
    fn new(dk: &KeyBytes, secret: &[u8], aad: &[u8]) -> Result<Self, CryptoError> {
        let salt = crypto::random::<SALT_LEN>();
        let kek = crypto::derive_key(secret, &salt)?;
        Ok(Wrap {
            salt: to_hex(&salt),
            key: crypto::seal(&kek, aad, dk)?,
        })
    }

    fn unwrap(&self, secret: &[u8], aad: &[u8]) -> Result<KeyBytes, CryptoError> {
        let salt = from_hex(&self.salt).ok_or(CryptoError)?;
        let kek = crypto::derive_key(secret, &salt)?;
        crypto::open(&kek, aad, &self.key)?
            .try_into()
            .map_err(|_| CryptoError)
    }
}

#[derive(Serialize, Deserialize)]
struct Recovery {
    question: String,
    wrap: Wrap,
}

#[derive(Serialize, Deserialize)]
struct VaultFile {
    version: u32,
    master: Wrap,
    recovery: Option<Recovery>,
    data: Sealed,
    /// Sealed `Vec<FileMeta>`. Absent in v2 files.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    files: Option<Sealed>,
}

#[derive(Serialize, Deserialize)]
struct BackupHeader {
    version: u32,
    master: Wrap,
    data: Sealed,
    #[serde(default)]
    files: Option<Sealed>,
    /// Blobs in the order they follow the header.
    #[serde(default)]
    blobs: Vec<BlobRef>,
}

#[derive(Serialize, Deserialize, Clone)]
struct BlobRef {
    id: u64,
    len: u64,
}

#[derive(Debug)]
pub enum VaultError {
    WrongPassword,
    WrongAnswer,
    NoRecovery,
    Locked,
    Corrupt,
    Invalid(String),
    Crypto,
    Io(io::Error),
}

impl fmt::Display for VaultError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VaultError::WrongPassword => write!(f, "Wrong master password"),
            VaultError::WrongAnswer => write!(f, "Wrong answer"),
            VaultError::NoRecovery => write!(f, "No recovery question set"),
            VaultError::Locked => write!(f, "Vault is locked"),
            VaultError::Corrupt => write!(f, "Vault file is corrupted"),
            VaultError::Invalid(msg) => write!(f, "{msg}"),
            VaultError::Crypto => write!(f, "Encryption failed"),
            VaultError::Io(e) => write!(f, "File error: {e}"),
        }
    }
}

impl From<io::Error> for VaultError {
    fn from(e: io::Error) -> Self {
        VaultError::Io(e)
    }
}

impl From<CryptoError> for VaultError {
    fn from(_: CryptoError) -> Self {
        VaultError::Crypto
    }
}

pub struct Vault {
    dir: PathBuf,
    /// Data key while unlocked. Cleared on lock.
    session: Mutex<Option<KeyBytes>>,
    /// Serializes read-modify-write cycles. File commands run off the main thread.
    io: Mutex<()>,
}

impl Vault {
    pub fn new(dir: PathBuf) -> Self {
        Self {
            dir,
            session: Mutex::new(None),
            io: Mutex::new(()),
        }
    }

    pub fn status(&self) -> Result<Status, VaultError> {
        let file = self.read()?;
        let exists = file.is_some() || self.legacy_path().exists();
        Ok(Status {
            exists,
            recovery_question: file.and_then(|f| f.recovery.map(|r| r.question)),
        })
    }

    /// Opens the vault. Creates it if missing. Migrates a legacy file if present.
    pub fn unlock(&self, password: &str) -> Result<Vec<Entry>, VaultError> {
        let _io = self.io();
        let (key, entries) = match self.read()? {
            Some(file) => {
                let key = file
                    .master
                    .unwrap(password.as_bytes(), AAD_MASTER)
                    .map_err(|_| VaultError::WrongPassword)?;
                let entries = open_entries(&key, &file.data)?;
                (key, entries)
            }
            None if self.legacy_path().exists() => self.migrate_legacy(password)?,
            None => {
                check_password(password)?;
                let key = crypto::random::<32>();
                self.write(&mut VaultFile {
                    version: FORMAT_VERSION,
                    master: Wrap::new(&key, password.as_bytes(), AAD_MASTER)?,
                    recovery: None,
                    data: seal_entries(&key, &[])?,
                    files: None,
                })?;
                (key, Vec::new())
            }
        };
        self.set_session(Some(key));
        Ok(entries)
    }

    pub fn lock(&self) {
        self.set_session(None);
    }

    pub fn save(&self, entries: &[Entry]) -> Result<(), VaultError> {
        let key = self.key()?;
        let _io = self.io();
        let mut file = self.read_required()?;
        file.data = seal_entries(&key, entries)?;
        // A deleted login's files stay in the vault, unlinked.
        let mut files = open_files(&key, file.files.as_ref())?;
        let mut unlinked = false;
        for f in &mut files {
            if f.entry_id.is_some_and(|id| !entries.iter().any(|e| e.id == id)) {
                f.entry_id = None;
                unlinked = true;
            }
        }
        if unlinked {
            file.files = Some(seal_files(&key, &files)?);
        }
        self.write(&mut file)
    }

    pub fn change_master(&self, password: &str) -> Result<(), VaultError> {
        check_password(password)?;
        let key = self.key()?;
        let _io = self.io();
        let mut file = self.read_required()?;
        file.master = Wrap::new(&key, password.as_bytes(), AAD_MASTER)?;
        self.write(&mut file)
    }

    pub fn set_recovery(&self, question: &str, answer: &str) -> Result<(), VaultError> {
        let question = question.trim();
        if question.is_empty() {
            return Err(VaultError::Invalid("Question is required".into()));
        }
        let answer = normalize_answer(answer)?;
        let key = self.key()?;
        let _io = self.io();
        let mut file = self.read_required()?;
        file.recovery = Some(Recovery {
            question: question.to_string(),
            wrap: Wrap::new(&key, answer.as_bytes(), AAD_RECOVERY)?,
        });
        self.write(&mut file)
    }

    /// Unlocks with the recovery answer and sets a new master password in one step.
    pub fn recover(&self, answer: &str, password: &str) -> Result<Vec<Entry>, VaultError> {
        check_password(password)?;
        let answer = normalize_answer(answer)?;
        let _io = self.io();
        let mut file = self.read_required()?;
        let key = file
            .recovery
            .as_ref()
            .ok_or(VaultError::NoRecovery)?
            .wrap
            .unwrap(answer.as_bytes(), AAD_RECOVERY)
            .map_err(|_| VaultError::WrongAnswer)?;
        let entries = open_entries(&key, &file.data)?;

        file.master = Wrap::new(&key, password.as_bytes(), AAD_MASTER)?;
        self.write(&mut file)?;
        self.set_session(Some(key));
        Ok(entries)
    }

    /// Removes every vault file and locks.
    pub fn delete(&self) -> Result<(), VaultError> {
        let _io = self.io();
        self.set_session(None);
        for path in [self.path(), self.legacy_path(), self.dir.join(BACKUP_FILE), self.tmp_path()] {
            ignore_missing(fs::remove_file(path))?;
        }
        ignore_missing(fs::remove_dir_all(self.files_dir()))?;
        Ok(())
    }

    pub fn list_files(&self) -> Result<Vec<FileMeta>, VaultError> {
        let key = self.key()?;
        let _io = self.io();
        open_files(&key, self.read_required()?.files.as_ref())
    }

    /// Encrypts a copy of the file at `src` into the vault.
    pub fn add_file(&self, src: &Path, entry_id: Option<u64>) -> Result<Vec<FileMeta>, VaultError> {
        let key = self.key()?;
        let _io = self.io();
        let name = src
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .ok_or_else(|| VaultError::Invalid("Pick a file".into()))?;
        if fs::metadata(src)?.len() > MAX_FILE_BYTES {
            return Err(VaultError::Invalid(format!(
                "Files over {} MB can't be stored",
                MAX_FILE_BYTES >> 20
            )));
        }
        let bytes = fs::read(src)?;

        let mut file = self.read_required()?;
        check_link(&key, &file, entry_id)?;
        let mut files = open_files(&key, file.files.as_ref())?;
        let meta = FileMeta {
            id: new_file_id(&files),
            name,
            size: bytes.len() as u64,
            sha256: to_hex(&Sha256::digest(&bytes)),
            added: now_ms(),
            entry_id,
        };

        let mut fresh = NewBlobs::new(self);
        fresh.write(&key, meta.id, &bytes)?;
        files.push(meta);
        file.files = Some(seal_files(&key, &files)?);
        self.write(&mut file)?;
        fresh.keep();
        Ok(files)
    }

    /// Decrypts a stored file to `dest`.
    pub fn save_file_copy(&self, id: u64, dest: &Path) -> Result<(), VaultError> {
        let key = self.key()?;
        let _io = self.io();
        let files = open_files(&key, self.read_required()?.files.as_ref())?;
        if !files.iter().any(|f| f.id == id) {
            return Err(file_missing());
        }
        let plain = self.read_blob(&key, id)?;
        write_atomic(dest, |w| w.write_all(&plain))?;
        Ok(())
    }

    pub fn remove_file(&self, id: u64) -> Result<Vec<FileMeta>, VaultError> {
        let key = self.key()?;
        let _io = self.io();
        let mut file = self.read_required()?;
        let mut files = open_files(&key, file.files.as_ref())?;
        let before = files.len();
        files.retain(|f| f.id != id);
        if files.len() == before {
            return Err(file_missing());
        }
        file.files = Some(seal_files(&key, &files)?);
        // Index first: a crash after this leaves an unreachable blob, not a broken index.
        self.write(&mut file)?;
        ignore_missing(fs::remove_file(self.blob_path(id)))?;
        Ok(files)
    }

    /// Links a file to a login, or unlinks it with `None`.
    pub fn link_file(&self, id: u64, entry_id: Option<u64>) -> Result<Vec<FileMeta>, VaultError> {
        let key = self.key()?;
        let _io = self.io();
        let mut file = self.read_required()?;
        check_link(&key, &file, entry_id)?;
        let mut files = open_files(&key, file.files.as_ref())?;
        files
            .iter_mut()
            .find(|f| f.id == id)
            .ok_or_else(file_missing)?
            .entry_id = entry_id;
        file.files = Some(seal_files(&key, &files)?);
        self.write(&mut file)?;
        Ok(files)
    }

    /// Writes a backup to `dest`. It opens with the current master password.
    /// The recovery wrap is left out.
    pub fn export(&self, dest: &Path) -> Result<(), VaultError> {
        let key = self.key()?;
        let _io = self.io();
        let file = self.read_required()?;
        let blobs = open_files(&key, file.files.as_ref())?
            .iter()
            .map(|f| {
                let len = fs::metadata(self.blob_path(f.id))?.len();
                Ok(BlobRef { id: f.id, len })
            })
            .collect::<io::Result<Vec<_>>>()?;
        let header = serde_json::to_vec(&BackupHeader {
            version: FORMAT_VERSION,
            master: file.master,
            data: file.data,
            files: file.files,
            blobs: blobs.clone(),
        })
        .map_err(|_| VaultError::Corrupt)?;
        let header_len = u32::try_from(header.len())
            .ok()
            .filter(|&n| n <= MAX_HEADER_BYTES)
            .ok_or(VaultError::Corrupt)?;

        write_atomic(dest, |w| {
            w.write_all(BACKUP_MAGIC)?;
            w.write_all(&header_len.to_le_bytes())?;
            w.write_all(&header)?;
            for b in &blobs {
                let copied = io::copy(&mut File::open(self.blob_path(b.id))?, w)?;
                if copied != b.len {
                    return Err(io::Error::other("a stored file changed during export"));
                }
            }
            Ok(())
        })?;
        Ok(())
    }

    /// Merges a backup, or a vault.json from another install, into this vault.
    /// Skips logins with the same site, username and password,
    /// and files with the same name and contents.
    pub fn import(&self, src: &Path, password: &str) -> Result<ImportSummary, VaultError> {
        let key = self.key()?;
        let mut backup = Backup::open(src)?;
        let theirs = backup
            .header
            .master
            .unwrap(password.as_bytes(), AAD_MASTER)
            .map_err(|_| VaultError::Invalid("Wrong password for this backup".into()))?;
        let their_entries = open_entries(&theirs, &backup.header.data).map_err(|_| damaged())?;
        let their_files = open_files(&theirs, backup.header.files.as_ref()).map_err(|_| damaged())?;

        let _io = self.io();
        let mut file = self.read_required()?;
        let mut entries = open_entries(&key, &file.data)?;
        let mut files = open_files(&key, file.files.as_ref())?;
        let (mut added_entries, mut added_files, mut skipped) = (0, 0, 0);

        // Login id in the backup -> id in this vault.
        let mut ids = HashMap::new();
        let mut taken: HashSet<u64> = entries.iter().map(|e| e.id).collect();
        for mut entry in their_entries {
            if let Some(same) = entries.iter().find(|e| same_login(e, &entry)) {
                ids.insert(entry.id, same.id);
                skipped += 1;
                continue;
            }
            let old = entry.id;
            if taken.contains(&entry.id) {
                entry.id = taken.iter().max().map_or(1, |m| m + 1);
            }
            taken.insert(entry.id);
            ids.insert(old, entry.id);
            entries.push(entry);
            added_entries += 1;
        }

        let mut fresh = NewBlobs::new(self);
        for mut meta in their_files {
            if files.iter().any(|f| f.name == meta.name && f.sha256 == meta.sha256) {
                skipped += 1;
                continue;
            }
            let plain = backup.blob(&theirs, meta.id)?;
            meta.id = new_file_id(&files);
            meta.entry_id = meta.entry_id.and_then(|id| ids.get(&id).copied());
            fresh.write(&key, meta.id, &plain)?;
            files.push(meta);
            added_files += 1;
        }

        file.data = seal_entries(&key, &entries)?;
        file.files = Some(seal_files(&key, &files)?);
        self.write(&mut file)?;
        fresh.keep();
        Ok(ImportSummary {
            entries,
            files,
            added_entries,
            added_files,
            skipped,
        })
    }

    fn migrate_legacy(&self, password: &str) -> Result<(KeyBytes, Vec<Entry>), VaultError> {
        let blob = fs::read(self.legacy_path())?;
        let plain = crypto::legacy_open(password, &blob).map_err(|_| VaultError::WrongPassword)?;
        let entries: Vec<Entry> = serde_json::from_slice(&plain).map_err(|_| VaultError::Corrupt)?;

        let key = crypto::random::<32>();
        // Write the new file before touching the old one.
        self.write(&mut VaultFile {
            version: FORMAT_VERSION,
            master: Wrap::new(&key, password.as_bytes(), AAD_MASTER)?,
            recovery: None,
            data: seal_entries(&key, &entries)?,
            files: None,
        })?;
        fs::rename(self.legacy_path(), self.dir.join(BACKUP_FILE))?;
        Ok((key, entries))
    }

    fn path(&self) -> PathBuf {
        self.dir.join(FILE)
    }

    fn tmp_path(&self) -> PathBuf {
        tmp_for(&self.path())
    }

    fn legacy_path(&self) -> PathBuf {
        self.dir.join(LEGACY_FILE)
    }

    fn files_dir(&self) -> PathBuf {
        self.dir.join(FILES_DIR)
    }

    fn blob_path(&self, id: u64) -> PathBuf {
        self.files_dir().join(format!("{id}.bin"))
    }

    fn read(&self) -> Result<Option<VaultFile>, VaultError> {
        match fs::read(self.path()) {
            Ok(bytes) => serde_json::from_slice(&bytes)
                .map(Some)
                .map_err(|_| VaultError::Corrupt),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn read_required(&self) -> Result<VaultFile, VaultError> {
        self.read()?.ok_or(VaultError::Corrupt)
    }

    fn write(&self, file: &mut VaultFile) -> Result<(), VaultError> {
        file.version = FORMAT_VERSION;
        fs::create_dir_all(&self.dir)?;
        let bytes = serde_json::to_vec(file).map_err(|_| VaultError::Corrupt)?;
        write_atomic(&self.path(), |w| w.write_all(&bytes))?;
        Ok(())
    }

    fn write_blob(&self, key: &KeyBytes, id: u64, plain: &[u8]) -> Result<(), VaultError> {
        fs::create_dir_all(self.files_dir())?;
        let blob = crypto::seal_raw(key, &blob_aad(id), plain)?;
        write_atomic(&self.blob_path(id), |w| w.write_all(&blob))?;
        Ok(())
    }

    fn read_blob(&self, key: &KeyBytes, id: u64) -> Result<Vec<u8>, VaultError> {
        let blob = fs::read(self.blob_path(id))?;
        crypto::open_raw(key, &blob_aad(id), blob).map_err(|_| VaultError::Corrupt)
    }

    fn key(&self) -> Result<KeyBytes, VaultError> {
        let guard = self.session.lock().unwrap_or_else(PoisonError::into_inner);
        guard.ok_or(VaultError::Locked)
    }

    fn set_session(&self, key: Option<KeyBytes>) {
        *self.session.lock().unwrap_or_else(PoisonError::into_inner) = key;
    }

    fn io(&self) -> MutexGuard<'_, ()> {
        self.io.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// Blobs written for an index change that is not saved yet. Deleted on drop unless kept.
struct NewBlobs<'a> {
    vault: &'a Vault,
    ids: Vec<u64>,
}

impl<'a> NewBlobs<'a> {
    fn new(vault: &'a Vault) -> Self {
        Self { vault, ids: Vec::new() }
    }

    fn write(&mut self, key: &KeyBytes, id: u64, plain: &[u8]) -> Result<(), VaultError> {
        self.ids.push(id);
        self.vault.write_blob(key, id, plain)
    }

    fn keep(mut self) {
        self.ids.clear();
    }
}

impl Drop for NewBlobs<'_> {
    fn drop(&mut self) {
        for &id in &self.ids {
            let _ = fs::remove_file(self.vault.blob_path(id));
        }
    }
}

/// Blob id -> (offset, length) inside a bundle.
type BlobOffsets = HashMap<u64, (u64, u64)>;

/// An opened backup: header in memory, blobs read on demand.
struct Backup {
    header: BackupHeader,
    /// Bundle file and where its blobs sit. None for a plain vault.json.
    body: Option<(File, BlobOffsets)>,
}

impl Backup {
    fn open(src: &Path) -> Result<Self, VaultError> {
        let mut f = File::open(src)?;
        let mut magic = [0u8; BACKUP_MAGIC.len()];
        if f.read_exact(&mut magic).is_err() || &magic != BACKUP_MAGIC {
            return Self::open_vault_json(src);
        }
        let mut len = [0u8; 4];
        f.read_exact(&mut len).map_err(|_| damaged())?;
        let len = u32::from_le_bytes(len);
        if len > MAX_HEADER_BYTES {
            return Err(damaged());
        }
        let mut head = vec![0u8; len as usize];
        f.read_exact(&mut head).map_err(|_| damaged())?;
        let header: BackupHeader = serde_json::from_slice(&head).map_err(|_| damaged())?;
        if header.version > FORMAT_VERSION {
            return Err(VaultError::Invalid(
                "This backup is from a newer version of Crypt".into(),
            ));
        }

        let mut at = (BACKUP_MAGIC.len() + 4) as u64 + u64::from(len);
        let mut offsets = HashMap::new();
        for b in &header.blobs {
            if b.len > MAX_FILE_BYTES + BLOB_OVERHEAD {
                return Err(damaged());
            }
            offsets.insert(b.id, (at, b.len));
            at += b.len;
        }
        Ok(Backup {
            header,
            body: Some((f, offsets)),
        })
    }

    /// A vault.json copied from another install. Its blobs are not in it, so logins only.
    fn open_vault_json(src: &Path) -> Result<Self, VaultError> {
        let not_backup = || VaultError::Invalid("Not a Crypt backup".into());
        if fs::metadata(src)?.len() > u64::from(MAX_HEADER_BYTES) {
            return Err(not_backup());
        }
        let vault: VaultFile = serde_json::from_slice(&fs::read(src)?).map_err(|_| not_backup())?;
        Ok(Backup {
            header: BackupHeader {
                version: vault.version,
                master: vault.master,
                data: vault.data,
                files: None,
                blobs: Vec::new(),
            },
            body: None,
        })
    }

    fn blob(&mut self, key: &KeyBytes, id: u64) -> Result<Vec<u8>, VaultError> {
        let (f, offsets) = self.body.as_mut().ok_or_else(damaged)?;
        let &(start, len) = offsets.get(&id).ok_or_else(damaged)?;
        f.seek(SeekFrom::Start(start))?;
        let mut blob = vec![0u8; len as usize];
        f.read_exact(&mut blob).map_err(|_| damaged())?;
        crypto::open_raw(key, &blob_aad(id), blob).map_err(|_| damaged())
    }
}

/// Atomic write: temp file next to the target, then rename over it.
fn write_atomic(
    path: &Path,
    fill: impl FnOnce(&mut BufWriter<File>) -> io::Result<()>,
) -> io::Result<()> {
    let tmp = tmp_for(path);
    let result = (|| {
        let mut out = BufWriter::new(File::create(&tmp)?);
        fill(&mut out)?;
        out.into_inner().map_err(io::IntoInnerError::into_error)?;
        fs::rename(&tmp, path)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}

fn tmp_for(path: &Path) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(".tmp");
    PathBuf::from(name)
}

fn ignore_missing(result: io::Result<()>) -> io::Result<()> {
    match result {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        other => other,
    }
}

fn check_password(password: &str) -> Result<(), VaultError> {
    if password.chars().count() < MIN_PASSWORD_LEN {
        return Err(VaultError::Invalid(format!(
            "Use at least {MIN_PASSWORD_LEN} characters"
        )));
    }
    Ok(())
}

/// Case-insensitive, whitespace-collapsed. "  Rex  " and "rex" match.
fn normalize_answer(answer: &str) -> Result<String, VaultError> {
    let norm = answer.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase();
    if norm.chars().count() < MIN_ANSWER_LEN {
        return Err(VaultError::Invalid(format!(
            "Answer needs at least {MIN_ANSWER_LEN} characters"
        )));
    }
    Ok(norm)
}

fn check_link(key: &KeyBytes, file: &VaultFile, entry_id: Option<u64>) -> Result<(), VaultError> {
    match entry_id {
        Some(id) if !open_entries(key, &file.data)?.iter().any(|e| e.id == id) => {
            Err(VaultError::Invalid("That login no longer exists".into()))
        }
        _ => Ok(()),
    }
}

fn same_login(a: &Entry, b: &Entry) -> bool {
    a.site == b.site && a.username == b.username && a.password == b.password
}

fn new_file_id(files: &[FileMeta]) -> u64 {
    loop {
        let id = u64::from_le_bytes(crypto::random::<8>()) >> 11;
        if id != 0 && !files.iter().any(|f| f.id == id) {
            return id;
        }
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as u64)
}

fn file_missing() -> VaultError {
    VaultError::Invalid("File not found in the vault".into())
}

fn damaged() -> VaultError {
    VaultError::Invalid("Backup file is damaged".into())
}

/// Binds a blob to its id, so blobs can't be swapped on disk.
fn blob_aad(id: u64) -> Vec<u8> {
    format!("crypt/v2/file/{id}").into_bytes()
}

fn seal_entries(key: &KeyBytes, entries: &[Entry]) -> Result<Sealed, VaultError> {
    let json = serde_json::to_vec(entries).map_err(|_| VaultError::Corrupt)?;
    Ok(crypto::seal(key, AAD_DATA, &json)?)
}

fn open_entries(key: &KeyBytes, data: &Sealed) -> Result<Vec<Entry>, VaultError> {
    let plain = crypto::open(key, AAD_DATA, data).map_err(|_| VaultError::Corrupt)?;
    serde_json::from_slice(&plain).map_err(|_| VaultError::Corrupt)
}

fn seal_files(key: &KeyBytes, files: &[FileMeta]) -> Result<Sealed, VaultError> {
    let json = serde_json::to_vec(files).map_err(|_| VaultError::Corrupt)?;
    Ok(crypto::seal(key, AAD_FILES, &json)?)
}

fn open_files(key: &KeyBytes, sealed: Option<&Sealed>) -> Result<Vec<FileMeta>, VaultError> {
    let Some(sealed) = sealed else {
        return Ok(Vec::new());
    };
    let plain = crypto::open(key, AAD_FILES, sealed).map_err(|_| VaultError::Corrupt)?;
    serde_json::from_slice(&plain).map_err(|_| VaultError::Corrupt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aes_gcm::{
        aead::{Aead, AeadCore, KeyInit, OsRng},
        Aes256Gcm, Key,
    };
    use pbkdf2::pbkdf2_hmac;
    use sha2::Sha256;

    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            static SEQ: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let seq = SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let dir = std::env::temp_dir().join(format!("crypt-test-{nanos}-{seq}"));
            TempDir(dir)
        }

        /// A file outside the vault folder, like one the user picks.
        fn outside(&self, name: &str, bytes: &[u8]) -> PathBuf {
            let dir = self.0.join("outside");
            fs::create_dir_all(&dir).unwrap();
            let path = dir.join(name);
            fs::write(&path, bytes).unwrap();
            path
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn entry(site: &str) -> Entry {
        Entry {
            id: 1,
            site: site.into(),
            username: "me".into(),
            password: "hunter2hunter2".into(),
            notes: String::new(),
            date: None,
        }
    }

    /// Writes a file the way the v1 app did.
    fn legacy_seal(password: &str, plain: &[u8]) -> Vec<u8> {
        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(password.as_bytes(), b"salt", 10_000, &mut key);
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
        let nonce = Aes256Gcm::generate_nonce(OsRng);
        let mut out = cipher.encrypt(&nonce, plain).unwrap();
        out.extend_from_slice(nonce.as_slice());
        out
    }

    #[test]
    fn roundtrip_and_wrong_password() {
        let tmp = TempDir::new();
        let vault = Vault::new(tmp.0.clone());
        assert!(vault.unlock("correct horse").unwrap().is_empty());
        vault.save(&[entry("github.com")]).unwrap();
        vault.lock();

        assert!(matches!(vault.unlock("wrong one!!"), Err(VaultError::WrongPassword)));
        let entries = vault.unlock("correct horse").unwrap();
        assert_eq!(entries[0].site, "github.com");
    }

    #[test]
    fn locked_vault_rejects_writes() {
        let tmp = TempDir::new();
        let vault = Vault::new(tmp.0.clone());
        vault.unlock("correct horse").unwrap();
        vault.lock();
        assert!(matches!(vault.save(&[]), Err(VaultError::Locked)));
    }

    #[test]
    fn recovery_sets_new_master() {
        let tmp = TempDir::new();
        let vault = Vault::new(tmp.0.clone());
        vault.unlock("old master pw").unwrap();
        vault.save(&[entry("mail.com")]).unwrap();
        vault.set_recovery("First pet?", "Rex").unwrap();
        vault.lock();

        assert_eq!(vault.status().unwrap().recovery_question.as_deref(), Some("First pet?"));
        assert!(matches!(vault.recover("Fluffy", "new master pw"), Err(VaultError::WrongAnswer)));

        let entries = vault.recover("  rex ", "new master pw").unwrap();
        assert_eq!(entries[0].site, "mail.com");
        vault.lock();

        assert!(matches!(vault.unlock("old master pw"), Err(VaultError::WrongPassword)));
        assert_eq!(vault.unlock("new master pw").unwrap().len(), 1);
    }

    #[test]
    fn change_master_keeps_recovery_working() {
        let tmp = TempDir::new();
        let vault = Vault::new(tmp.0.clone());
        vault.unlock("first master").unwrap();
        vault.set_recovery("City?", "Paris").unwrap();
        vault.change_master("second master").unwrap();
        vault.lock();

        assert!(vault.unlock("second master").is_ok());
        vault.lock();
        assert!(vault.recover("paris", "third master").is_ok());
    }

    #[test]
    fn legacy_file_migrates() {
        let tmp = TempDir::new();
        fs::create_dir_all(&tmp.0).unwrap();
        let json = serde_json::to_vec(&vec![entry("old-site.org")]).unwrap();
        fs::write(tmp.0.join(LEGACY_FILE), legacy_seal("legacy master", &json)).unwrap();

        let vault = Vault::new(tmp.0.clone());
        assert!(vault.status().unwrap().exists);
        let entries = vault.unlock("legacy master").unwrap();
        assert_eq!(entries[0].site, "old-site.org");
        assert!(!tmp.0.join(LEGACY_FILE).exists());
        assert!(tmp.0.join(BACKUP_FILE).exists());

        vault.lock();
        assert_eq!(vault.unlock("legacy master").unwrap().len(), 1);
    }

    #[test]
    fn short_master_rejected_on_create() {
        let tmp = TempDir::new();
        let vault = Vault::new(tmp.0.clone());
        assert!(matches!(vault.unlock("short"), Err(VaultError::Invalid(_))));
    }

    #[test]
    fn file_roundtrip_is_encrypted_on_disk() {
        let tmp = TempDir::new();
        let vault = Vault::new(tmp.0.clone());
        vault.unlock("correct horse").unwrap();
        let plain = b"recovery codes: 1111 2222 3333".repeat(100);
        let files = vault.add_file(&tmp.outside("codes.txt", &plain), None).unwrap();
        let meta = &files[0];
        assert_eq!(meta.name, "codes.txt");
        assert_eq!(meta.size, plain.len() as u64);
        vault.lock();

        let blob = fs::read(vault.blob_path(meta.id)).unwrap();
        assert_eq!(blob.len() as u64, meta.size + BLOB_OVERHEAD);
        assert!(!blob.windows(14).any(|w| w == b"recovery codes"));
        assert!(matches!(vault.list_files(), Err(VaultError::Locked)));

        vault.unlock("correct horse").unwrap();
        let out = tmp.0.join("outside").join("copy.txt");
        vault.save_file_copy(meta.id, &out).unwrap();
        assert_eq!(fs::read(out).unwrap(), plain);
    }

    #[test]
    fn oversized_file_rejected() {
        let tmp = TempDir::new();
        let vault = Vault::new(tmp.0.clone());
        vault.unlock("correct horse").unwrap();
        let big = tmp.outside("big.bin", b"");
        File::options().write(true).open(&big).unwrap().set_len(MAX_FILE_BYTES + 1).unwrap();
        assert!(matches!(vault.add_file(&big, None), Err(VaultError::Invalid(_))));
        assert!(vault.list_files().unwrap().is_empty());
    }

    #[test]
    fn remove_and_delete_clean_up_blobs() {
        let tmp = TempDir::new();
        let vault = Vault::new(tmp.0.clone());
        vault.unlock("correct horse").unwrap();
        vault.add_file(&tmp.outside("a.txt", b"aaa"), None).unwrap();
        let files = vault.add_file(&tmp.outside("b.txt", b"bbb"), None).unwrap();
        let (a, b) = (files[0].id, files[1].id);

        let left = vault.remove_file(a).unwrap();
        assert_eq!(left.len(), 1);
        assert!(!vault.blob_path(a).exists());
        assert!(vault.blob_path(b).exists());
        assert!(matches!(vault.remove_file(a), Err(VaultError::Invalid(_))));

        vault.delete().unwrap();
        assert!(!vault.files_dir().exists());
    }

    #[test]
    fn deleting_a_login_unlinks_its_files() {
        let tmp = TempDir::new();
        let vault = Vault::new(tmp.0.clone());
        vault.unlock("correct horse").unwrap();
        vault.save(&[entry("github.com")]).unwrap();
        assert!(matches!(
            vault.add_file(&tmp.outside("key.txt", b"k"), Some(99)),
            Err(VaultError::Invalid(_))
        ));
        let files = vault.add_file(&tmp.outside("key.txt", b"k"), Some(1)).unwrap();
        assert_eq!(files[0].entry_id, Some(1));

        vault.save(&[]).unwrap();
        let files = vault.list_files().unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].entry_id, None);
    }

    #[test]
    fn link_file_sets_and_clears() {
        let tmp = TempDir::new();
        let vault = Vault::new(tmp.0.clone());
        vault.unlock("correct horse").unwrap();
        vault.save(&[entry("github.com")]).unwrap();
        let id = vault.add_file(&tmp.outside("f.txt", b"f"), None).unwrap()[0].id;
        assert_eq!(vault.link_file(id, Some(1)).unwrap()[0].entry_id, Some(1));
        assert_eq!(vault.link_file(id, None).unwrap()[0].entry_id, None);
        assert!(matches!(vault.link_file(id, Some(7)), Err(VaultError::Invalid(_))));
    }

    #[test]
    fn export_then_import_into_another_vault() {
        let src_dir = TempDir::new();
        let src = Vault::new(src_dir.0.clone());
        src.unlock("alpha master").unwrap();
        src.save(&[entry("a.com"), Entry { id: 2, ..entry("shared.com") }]).unwrap();
        src.set_recovery("Pet?", "Rex").unwrap();
        src.add_file(&src_dir.outside("doc.pdf", b"%PDF-fake"), Some(1)).unwrap();
        let backup = src_dir.0.join("outside").join("vault.cryptbak");
        src.export(&backup).unwrap();

        let dst_dir = TempDir::new();
        let dst = Vault::new(dst_dir.0.clone());
        dst.unlock("beta master").unwrap();
        // Same id as a.com in the backup, different login: the import must renumber.
        dst.save(&[entry("own.org"), Entry { id: 5, ..entry("shared.com") }]).unwrap();

        assert!(matches!(dst.import(&backup, "beta master"), Err(VaultError::Invalid(_))));

        let sum = dst.import(&backup, "alpha master").unwrap();
        assert_eq!((sum.added_entries, sum.added_files, sum.skipped), (1, 1, 1));
        assert_eq!(sum.entries.len(), 3);
        let a = sum.entries.iter().find(|e| e.site == "a.com").unwrap();
        assert_ne!(a.id, 1);
        assert_eq!(sum.files[0].entry_id, Some(a.id));

        let out = src_dir.0.join("outside").join("restored.pdf");
        dst.save_file_copy(sum.files[0].id, &out).unwrap();
        assert_eq!(fs::read(out).unwrap(), b"%PDF-fake");

        let again = dst.import(&backup, "alpha master").unwrap();
        assert_eq!((again.added_entries, again.added_files, again.skipped), (0, 0, 3));

        dst.lock();
        assert_eq!(dst.unlock("beta master").unwrap().len(), 3);
    }

    #[test]
    fn import_plain_vault_json() {
        let src_dir = TempDir::new();
        let src = Vault::new(src_dir.0.clone());
        src.unlock("alpha master").unwrap();
        src.save(&[entry("a.com")]).unwrap();
        src.add_file(&src_dir.outside("x.txt", b"x"), None).unwrap();

        let dst_dir = TempDir::new();
        let dst = Vault::new(dst_dir.0.clone());
        dst.unlock("beta master").unwrap();
        let sum = dst.import(&src.path(), "alpha master").unwrap();
        assert_eq!((sum.added_entries, sum.added_files), (1, 0));

        let junk = dst_dir.outside("junk.txt", b"not a vault");
        assert!(matches!(dst.import(&junk, "alpha master"), Err(VaultError::Invalid(_))));
    }

    #[test]
    fn v2_file_without_index_still_opens() {
        let tmp = TempDir::new();
        let vault = Vault::new(tmp.0.clone());
        vault.unlock("correct horse").unwrap();
        vault.save(&[entry("github.com")]).unwrap();
        vault.lock();

        let mut json: serde_json::Value = serde_json::from_slice(&fs::read(vault.path()).unwrap()).unwrap();
        json["version"] = 2.into();
        assert!(json.get("files").is_none());
        fs::write(vault.path(), serde_json::to_vec(&json).unwrap()).unwrap();

        assert_eq!(vault.unlock("correct horse").unwrap().len(), 1);
        assert!(vault.list_files().unwrap().is_empty());
    }
}
