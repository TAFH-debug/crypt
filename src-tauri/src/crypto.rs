//! Primitives only: key derivation, AES-256-GCM sealing, hex encoding.
//! Nothing here knows about vault files.

use aes_gcm::{
    aead::{rand_core::RngCore, Aead, AeadCore, KeyInit, OsRng, Payload},
    Aes256Gcm, Key, Nonce,
};
use argon2::Argon2;
use pbkdf2::pbkdf2_hmac;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

pub type KeyBytes = [u8; 32];
pub const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const TAG_LEN: usize = 16;

#[derive(Debug)]
pub struct CryptoError;

/// Nonce and ciphertext (tag included), both hex encoded.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sealed {
    pub nonce: String,
    pub ct: String,
}

pub fn random<const N: usize>() -> [u8; N] {
    let mut buf = [0u8; N];
    OsRng.fill_bytes(&mut buf);
    buf
}

/// Argon2id with the crate defaults. Slow on purpose.
pub fn derive_key(secret: &[u8], salt: &[u8]) -> Result<KeyBytes, CryptoError> {
    let mut out = [0u8; 32];
    Argon2::default()
        .hash_password_into(secret, salt, &mut out)
        .map_err(|_| CryptoError)?;
    Ok(out)
}

pub fn seal(key: &KeyBytes, aad: &[u8], plain: &[u8]) -> Result<Sealed, CryptoError> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Aes256Gcm::generate_nonce(OsRng);
    let ct = cipher
        .encrypt(&nonce, Payload { msg: plain, aad })
        .map_err(|_| CryptoError)?;
    Ok(Sealed {
        nonce: to_hex(nonce.as_slice()),
        ct: to_hex(&ct),
    })
}

pub fn open(key: &KeyBytes, aad: &[u8], sealed: &Sealed) -> Result<Vec<u8>, CryptoError> {
    let nonce = from_hex(&sealed.nonce)
        .filter(|n| n.len() == NONCE_LEN)
        .ok_or(CryptoError)?;
    let ct = from_hex(&sealed.ct).ok_or(CryptoError)?;
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    cipher
        .decrypt(Nonce::from_slice(&nonce), Payload { msg: &ct, aad })
        .map_err(|_| CryptoError)
}

/// Format v1 (the original app): `ciphertext || nonce`, key = PBKDF2(password, "salt", 10k).
/// Read-only, used once to migrate old vaults.
pub fn legacy_open(password: &str, blob: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if blob.len() < NONCE_LEN + TAG_LEN {
        return Err(CryptoError);
    }
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), b"salt", 10_000, &mut key);
    let (ct, nonce) = blob.split_at(blob.len() - NONCE_LEN);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
    cipher
        .decrypt(Nonce::from_slice(nonce), ct)
        .map_err(|_| CryptoError)
}

pub fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn from_hex(s: &str) -> Option<Vec<u8>> {
    if !s.len().is_multiple_of(2) {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(s.get(i..i + 2)?, 16).ok())
        .collect()
}
