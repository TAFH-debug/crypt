use core::str;
use std::{fs::*, io::{Read, Write}, path::{Path, PathBuf}};

use aes_gcm::{aead::{AeadMutInPlace, OsRng}, AeadCore, Aes256Gcm, Key, KeyInit, Nonce};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

fn pbkdf2(password: &str) -> [u8; 32] {
    let salt = b"salt";
    let n = 10_000;
    let mut key1 = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, n, &mut key1);
    key1
}

fn encrypt(plaintext: &str, hs: &[u8; 32]) -> Vec<u8> {
    let key = Key::<Aes256Gcm>::from_slice(hs);
    let mut cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(OsRng);

    let mut buffer = Vec::new();
    buffer.extend_from_slice(plaintext.as_bytes());
    let _ = cipher.encrypt_in_place(&nonce, b"", &mut buffer);
    buffer.append(&mut nonce.as_slice().to_vec());
    buffer
}

fn decrypt(ct: Vec<u8>, hs: &[u8; 32]) -> Vec<u8> {
    let key = Key::<Aes256Gcm>::from_slice(hs);
    let mut cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(&ct[ct.len()-12..]);

    let mut buffer = Vec::new();
    buffer.extend_from_slice(&ct[..ct.len()-12]);
    let _ = cipher.decrypt_in_place(&nonce, b"", &mut buffer);
    buffer
}

#[tauri::command]
fn save_store(store: serde_json::Value, password: &str) {
    let path: PathBuf = dirs_next::home_dir().unwrap().join("AppData\\Local\\com.crypt.app\\passwords.bin");
    let mut file = File::create(path.as_path()).unwrap();
    let sval = serde_json::to_string(&store).unwrap();

    let hs = pbkdf2(password);

    let ct = encrypt(sval.as_str(), &hs);
    file.write_all(&ct);
}

#[tauri::command]
fn get_store(password: &str) -> serde_json::Value {
    let path: PathBuf = dirs_next::home_dir().unwrap().join("AppData\\Local\\com.crypt.app\\passwords.bin");
    if !Path::new(path.as_path()).exists() {
        save_store(serde_json::json!([]), password);
    }

    let hs = pbkdf2(password);
    let mut s = OpenOptions::new().write(true).read(true).create(true).open(path.as_path()).unwrap();
    let mut buf = Vec::new();
    s.read_to_end(&mut buf);
    let m = decrypt(buf, &hs);
    let json: serde_json::Value = serde_json::from_str(str::from_utf8(&m).unwrap()).unwrap();
    json
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_store, save_store])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
