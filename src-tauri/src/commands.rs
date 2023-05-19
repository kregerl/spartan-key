use std::path::Path;

use tauri::Manager;

use crate::state::ConfigState;

#[tauri::command]
pub fn create_new_vault(vault_name: String, vault_path: String, master_password: String, app_handle: tauri::AppHandle<tauri::Wry>) {
    println!("vault_name: {}", vault_name);
    println!("vault_path: {}", vault_path);
    println!("master_password: {}", master_password);

    let config_state: tauri::State<ConfigState> = app_handle.state();
    config_state.add_vault(&vault_name, Path::new(&vault_path));
}