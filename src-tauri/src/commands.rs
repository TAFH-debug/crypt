//! Tauri command layer. Thin: maps vault results to strings for the frontend.

use std::path::Path;

use tauri::State;

use crate::vault::{Entry, FileMeta, ImportSummary, Status, Vault};

type CmdResult<T> = Result<T, String>;

fn to_msg<T, E: ToString>(r: Result<T, E>) -> CmdResult<T> {
    r.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn vault_status(vault: State<'_, Vault>) -> CmdResult<Status> {
    to_msg(vault.status())
}

#[tauri::command]
pub fn unlock(password: String, vault: State<'_, Vault>) -> CmdResult<Vec<Entry>> {
    to_msg(vault.unlock(&password))
}

#[tauri::command]
pub fn lock(vault: State<'_, Vault>) {
    vault.lock();
}

#[tauri::command]
pub fn save_store(store: Vec<Entry>, vault: State<'_, Vault>) -> CmdResult<()> {
    to_msg(vault.save(&store))
}

#[tauri::command]
pub fn change_master_password(password: String, vault: State<'_, Vault>) -> CmdResult<()> {
    to_msg(vault.change_master(&password))
}

#[tauri::command]
pub fn set_recovery(question: String, answer: String, vault: State<'_, Vault>) -> CmdResult<()> {
    to_msg(vault.set_recovery(&question, &answer))
}

#[tauri::command]
pub fn recover(answer: String, password: String, vault: State<'_, Vault>) -> CmdResult<Vec<Entry>> {
    to_msg(vault.recover(&answer, &password))
}

#[tauri::command]
pub fn delete_database(vault: State<'_, Vault>) -> CmdResult<()> {
    to_msg(vault.delete())
}

#[tauri::command]
pub fn list_files(vault: State<'_, Vault>) -> CmdResult<Vec<FileMeta>> {
    to_msg(vault.list_files())
}

// File and backup commands are async so large files don't block the main thread.

#[tauri::command]
pub async fn add_file(path: String, entry_id: Option<u64>, vault: State<'_, Vault>) -> CmdResult<Vec<FileMeta>> {
    to_msg(vault.add_file(Path::new(&path), entry_id))
}

#[tauri::command]
pub async fn save_file_copy(id: u64, path: String, vault: State<'_, Vault>) -> CmdResult<()> {
    to_msg(vault.save_file_copy(id, Path::new(&path)))
}

#[tauri::command]
pub fn remove_file(id: u64, vault: State<'_, Vault>) -> CmdResult<Vec<FileMeta>> {
    to_msg(vault.remove_file(id))
}

#[tauri::command]
pub fn link_file(id: u64, entry_id: Option<u64>, vault: State<'_, Vault>) -> CmdResult<Vec<FileMeta>> {
    to_msg(vault.link_file(id, entry_id))
}

#[tauri::command]
pub async fn export_vault(path: String, vault: State<'_, Vault>) -> CmdResult<()> {
    to_msg(vault.export(Path::new(&path)))
}

#[tauri::command]
pub async fn import_vault(path: String, password: String, vault: State<'_, Vault>) -> CmdResult<ImportSummary> {
    to_msg(vault.import(Path::new(&path), &password))
}
