// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::ser::StdError;
use std::{fs};
use tauri::{Manager, App, Wry};

use crate::{state::ConfigState, vault::{create_new_vault, VaultManagerState, add_entry, get_active_vault_entries, get_vaults}};

mod state;
mod vault;
mod error;

fn main() {
    tauri::Builder::default()
        .setup(setup)
        .on_window_event(|window_event| {
            if let tauri::WindowEvent::CloseRequested { .. } = window_event.event() {
                // On app shutdown save the (possibly) modified config file
                let app_handle = window_event.window().app_handle();
                println!("CloseRequested");
                let config_state: tauri::State<ConfigState> = app_handle.state();
                println!("Vaults: {:#?}", config_state.state.lock().unwrap());
                config_state.write().expect("Error writing config to file.");
            }
        })
        .invoke_handler(tauri::generate_handler![create_new_vault, add_entry, get_active_vault_entries, get_vaults])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App<Wry>) -> Result<(), Box<(dyn StdError + 'static)>> {
    // Setup app dir if it doesnt exist.
    let path_resolver = app.path_resolver();

    let app_dir = path_resolver.app_config_dir().unwrap();
    fs::create_dir_all(&app_dir)?;

    // Start managing the config state
    app.manage(ConfigState::new(&app_dir));
    app.manage(VaultManagerState::new());

    println!("After manage");
    Ok(())
}

