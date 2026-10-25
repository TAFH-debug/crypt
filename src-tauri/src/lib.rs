mod commands;
mod crypto;
mod vault;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Same folder as before: %LOCALAPPDATA%\com.crypt.app
            let dir = app.path().app_local_data_dir()?;
            app.manage(vault::Vault::new(dir));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::vault_status,
            commands::unlock,
            commands::lock,
            commands::save_store,
            commands::change_master_password,
            commands::set_recovery,
            commands::recover,
            commands::delete_database,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
