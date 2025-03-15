use std::{fs::*, path::Path};

#[tauri::command]
fn save_store(store: serde_json::Value) {
    println!("save_store");
    let file = File::create("passwords.json").unwrap();
    serde_json::to_writer(file, &store).unwrap();
}

#[tauri::command]
fn get_store() -> serde_json::Value {
    println!("get_store");
    if !Path::new("passwords.json").exists() {
        let file = File::create("passwords.json").unwrap();
        let json = serde_json::json!([]);
        serde_json::to_writer(file, &json).unwrap();
    }

    let s = read_to_string("passwords.json").unwrap();
    let json: serde_json::Value = serde_json::from_str(s.as_str()).unwrap();
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
