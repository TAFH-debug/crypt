//! Tauri command layer. Thin: maps vault results to strings for the frontend.

use tauri::State;

use crate::vault::{Entry, Status, Vault};

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
