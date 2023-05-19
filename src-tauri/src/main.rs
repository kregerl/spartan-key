// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pbkdf2::pbkdf2_hmac_array;
use rand::{seq::SliceRandom, Rng};
use serde::ser::StdError;
use sha2::Sha256;
use std::{fs, string::FromUtf8Error};
use tauri::Manager;

use crate::{commands::create_new_vault, state::ConfigState};

mod commands;
mod state;

fn main() {
    tauri::Builder::default()
        .setup(setup)
        .on_window_event(|window_event| {
            if let tauri::WindowEvent::CloseRequested { .. } = window_event.event() {
                // On
                let app_handle = window_event.window().app_handle();
                println!("CloseRequested");
                let config_state: tauri::State<ConfigState> = app_handle.state();
                println!("Vaults: {:#?}", config_state.state.lock().unwrap());
                config_state.write().expect("Error writing config to file.");
            }
        })
        .invoke_handler(tauri::generate_handler![create_new_vault])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut tauri::App<tauri::Wry>) -> Result<(), Box<(dyn StdError + 'static)>> {
    // Setup app dir if it doesnt exist.
    let path_resolver = app.path_resolver();

    let app_dir = path_resolver.app_config_dir().unwrap();
    fs::create_dir_all(&app_dir)?;

    // Start managing the config state
    app.manage(ConfigState::new(&app_dir));
    println!("After manage");
    Ok(())
}

/// Derive the encryption key from the master password and user's email address.
/// The email is treated as an "add" to make the hash tougher to break and is concatenated with the
/// master_password to form the "password" which the encryption key is derived from.
///
/// Automatically generate a 128-bit salt and perform SHA512 + PBKDF2 + HMAC with 100k iterations
///
/// Returns a tuple of `(256-bit encryption key, salt)`
const KEY_SIZE: usize = 32;
const SALT_SIZE: usize = 16;
fn derive_encryption_key(
    master_password: &str,
    predefined_salt: Option<[u8; SALT_SIZE]>,
) -> ([u8; KEY_SIZE], [u8; SALT_SIZE]) {
    const ITERATIONS: u32 = 100_000;
    let salt = match predefined_salt {
        Some(s) => s,
        None => rand::thread_rng().gen::<[u8; SALT_SIZE]>(),
    };
    (
        pbkdf2_hmac_array::<Sha256, KEY_SIZE>(master_password.as_bytes(), &salt, ITERATIONS),
        salt,
    )
}

/// Generates a random password that satisfies the following password requirements:
/// - At least one uppercase letter
/// - At least one digit
/// - At least one special character
///
/// Randomly generate how many of each char there should be, then randomly select that
/// many characters from the valid characters slices and put into a vec.
///
/// Finally, shuffle the vec and convert to utf8 ascii characters
fn generate_password(password_size: usize) -> Result<String, FromUtf8Error> {
    let lowercase_chars = "abcdefghijklmnopqrstuvwxyz".as_bytes();
    let uppercase_chars = "ABCDEFGHIJKLMNOPQRSTYVWXYZ".as_bytes();
    let special_chars = "!@#$%^&*".as_bytes();
    let digits = "01234567890".as_bytes();

    let mut rng = rand::thread_rng();
    let num_lowercase = rng.gen_range(1..password_size - 3);
    let num_uppercase = rng.gen_range(1..password_size - num_lowercase - 2);
    let num_digits = rng.gen_range(1..password_size - num_lowercase - num_uppercase - 1);
    let num_special = password_size - (num_lowercase + num_uppercase + num_digits);

    let mut bytes: Vec<u8> = Vec::with_capacity(password_size);
    for _ in 0..num_lowercase {
        bytes.push(lowercase_chars[rng.gen_range(0..lowercase_chars.len())]);
    }

    for _ in 0..num_uppercase {
        bytes.push(uppercase_chars[rng.gen_range(0..uppercase_chars.len())]);
    }

    for _ in 0..num_digits {
        bytes.push(digits[rng.gen_range(0..digits.len())]);
    }

    for _ in 0..num_special {
        bytes.push(special_chars[rng.gen_range(0..special_chars.len())]);
    }
    bytes.shuffle(&mut rng);

    String::from_utf8(bytes)
}
