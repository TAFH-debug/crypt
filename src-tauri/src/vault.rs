//! Vault file format and session handling.
//!
//! A random data key (DK) encrypts the entries. The DK is stored twice:
//! wrapped by the master password, and optionally wrapped by the recovery answer.
//! Changing the master password only rewraps the DK; entries are not re-encrypted.

use std::{
    fmt, fs, io,
    path::PathBuf,
    sync::{Mutex, PoisonError},
};

use serde::{Deserialize, Serialize};

use crate::crypto::{self, from_hex, to_hex, CryptoError, KeyBytes, Sealed, SALT_LEN};

const FORMAT_VERSION: u32 = 2;
const FILE: &str = "vault.json";
const LEGACY_FILE: &str = "passwords.bin";
const BACKUP_FILE: &str = "passwords.bin.bak";
const MIN_PASSWORD_LEN: usize = 8;
const MIN_ANSWER_LEN: usize = 3;

const AAD_MASTER: &[u8] = b"crypt/v2/master";
const AAD_RECOVERY: &[u8] = b"crypt/v2/recovery";
const AAD_DATA: &[u8] = b"crypt/v2/data";

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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub exists: bool,
    pub recovery_question: Option<String>,
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
}

impl Vault {
    pub fn new(dir: PathBuf) -> Self {
        Self {
            dir,
            session: Mutex::new(None),
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
                self.write(&VaultFile {
                    version: FORMAT_VERSION,
                    master: Wrap::new(&key, password.as_bytes(), AAD_MASTER)?,
                    recovery: None,
                    data: seal_entries(&key, &[])?,
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
        let mut file = self.read_required()?;
        file.data = seal_entries(&key, entries)?;
        self.write(&file)
    }

    pub fn change_master(&self, password: &str) -> Result<(), VaultError> {
        check_password(password)?;
        let key = self.key()?;
        let mut file = self.read_required()?;
        file.master = Wrap::new(&key, password.as_bytes(), AAD_MASTER)?;
        self.write(&file)
    }

    pub fn set_recovery(&self, question: &str, answer: &str) -> Result<(), VaultError> {
        let question = question.trim();
        if question.is_empty() {
            return Err(VaultError::Invalid("Question is required".into()));
        }
        let answer = normalize_answer(answer)?;
        let key = self.key()?;
        let mut file = self.read_required()?;
        file.recovery = Some(Recovery {
            question: question.to_string(),
            wrap: Wrap::new(&key, answer.as_bytes(), AAD_RECOVERY)?,
        });
        self.write(&file)
    }

    /// Unlocks with the recovery answer and sets a new master password in one step.
    pub fn recover(&self, answer: &str, password: &str) -> Result<Vec<Entry>, VaultError> {
        check_password(password)?;
        let answer = normalize_answer(answer)?;
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
        self.write(&file)?;
        self.set_session(Some(key));
        Ok(entries)
    }

    /// Removes every vault file and locks.
    pub fn delete(&self) -> Result<(), VaultError> {
        self.set_session(None);
        for path in [self.path(), self.legacy_path(), self.dir.join(BACKUP_FILE), self.tmp_path()] {
            match fs::remove_file(path) {
                Ok(()) => {}
                Err(e) if e.kind() == io::ErrorKind::NotFound => {}
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }

    fn migrate_legacy(&self, password: &str) -> Result<(KeyBytes, Vec<Entry>), VaultError> {
        let blob = fs::read(self.legacy_path())?;
        let plain = crypto::legacy_open(password, &blob).map_err(|_| VaultError::WrongPassword)?;
        let entries: Vec<Entry> = serde_json::from_slice(&plain).map_err(|_| VaultError::Corrupt)?;

        let key = crypto::random::<32>();
        // Write the new file before touching the old one.
        self.write(&VaultFile {
            version: FORMAT_VERSION,
            master: Wrap::new(&key, password.as_bytes(), AAD_MASTER)?,
            recovery: None,
            data: seal_entries(&key, &entries)?,
        })?;
        fs::rename(self.legacy_path(), self.dir.join(BACKUP_FILE))?;
        Ok((key, entries))
    }

    fn path(&self) -> PathBuf {
        self.dir.join(FILE)
    }

    fn tmp_path(&self) -> PathBuf {
        self.dir.join("vault.json.tmp")
    }

    fn legacy_path(&self) -> PathBuf {
        self.dir.join(LEGACY_FILE)
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

    /// Atomic write: temp file, then rename over the target.
    fn write(&self, file: &VaultFile) -> Result<(), VaultError> {
        fs::create_dir_all(&self.dir)?;
        let bytes = serde_json::to_vec(file).map_err(|_| VaultError::Corrupt)?;
        fs::write(self.tmp_path(), bytes)?;
        fs::rename(self.tmp_path(), self.path())?;
        Ok(())
    }

    fn key(&self) -> Result<KeyBytes, VaultError> {
        let guard = self.session.lock().unwrap_or_else(PoisonError::into_inner);
        guard.ok_or(VaultError::Locked)
    }

    fn set_session(&self, key: Option<KeyBytes>) {
        *self.session.lock().unwrap_or_else(PoisonError::into_inner) = key;
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

fn seal_entries(key: &KeyBytes, entries: &[Entry]) -> Result<Sealed, VaultError> {
    let json = serde_json::to_vec(entries).map_err(|_| VaultError::Corrupt)?;
    Ok(crypto::seal(key, AAD_DATA, &json)?)
}

fn open_entries(key: &KeyBytes, data: &Sealed) -> Result<Vec<Entry>, VaultError> {
    let plain = crypto::open(key, AAD_DATA, data).map_err(|_| VaultError::Corrupt)?;
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
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let dir = std::env::temp_dir().join(format!("crypt-test-{nanos}"));
            TempDir(dir)
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
}
