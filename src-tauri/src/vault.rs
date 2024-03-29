use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufReader, Read},
    path::{Path, PathBuf},
    sync::Mutex,
};

use aes_gcm::{
    aead::{generic_array::GenericArray, Aead},
    Aes256Gcm, KeyInit, Nonce,
};
use bincode::ErrorKind;
use pbkdf2::pbkdf2_hmac_array;
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use sha2::{digest::typenum, Sha256};
use tauri::Manager;

use crate::{error::EncryptionResult, state::ConfigState};

const SALT_SIZE: usize = 16;
const NONCE_SIZE: usize = 12;
const KEY_SIZE: usize = 32;

pub struct VaultManagerState(pub Mutex<VaultManager>);

impl VaultManagerState {
    pub fn new() -> Self {
        Self(Mutex::new(VaultManager {
            active_vault_name: None,
            vaults: HashMap::default(),
        }))
    }
}
pub struct VaultManager {
    active_vault_name: Option<String>,
    vaults: HashMap<String, Vault>,
}

impl VaultManager {
    pub fn add_and_activate_vault(&mut self, vault_name: &str, vault: Vault) {
        self.add_vault(vault_name.into(), vault);
        self.set_active_vault(vault_name.into());
    }

    pub fn add_vault(&mut self, vault_name: String, vault: Vault) {
        self.vaults.insert(vault_name, vault);
    }

    pub fn set_active_vault(&mut self, vault_name: String) {
        self.active_vault_name = Some(vault_name);
    }

    pub fn get_vault(&self, name: &str) -> Option<&Vault> {
        self.vaults.get(name)
    }

    pub fn get_active_vault_name(&self) -> Option<&str> {
        self.active_vault_name.as_deref()
    }

    pub fn get_active_vault(&mut self) -> Option<&mut Vault> {
        let x = self.active_vault_name.as_deref()?;
        self.vaults.get_mut(x)
    }

    pub fn get_vaults(&self) -> &HashMap<String, Vault> {
        &self.vaults
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct VaultHeader {
    signature: [u8; 1],
    salt: [u8; SALT_SIZE],
    master_password_nonce: [u8; NONCE_SIZE],
    recovery_key_nonce: [u8; NONCE_SIZE],
    master_password_key: Vec<u8>,
    recovery_key: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct Vault {
    // Header info
    header: VaultHeader,
    // Do not serialize
    internal_key: [u8; KEY_SIZE],
    path: PathBuf,
    // Encrypted Data
    vault_entries: HashMap<String, VaultEntry>,
}

impl Vault {
    pub fn new(
        path: &Path,
        salt: [u8; SALT_SIZE],
        master_password_nonce: [u8; NONCE_SIZE],
        recovery_key_nonce: [u8; NONCE_SIZE],
        internal_key: [u8; KEY_SIZE],
        master_password_key: Vec<u8>,
        recovery_key: Vec<u8>,
    ) -> Self {
        Self {
            header: VaultHeader {
                signature: [0xED],
                salt,
                master_password_nonce,
                recovery_key_nonce,
                master_password_key,
                recovery_key,
            },
            path: path.into(),
            internal_key,
            vault_entries: HashMap::default(),
        }
    }

    pub fn read(path: &Path, master_password: &str) -> Result<Self, Box<ErrorKind>> {
        let mut reader: BufReader<File> = BufReader::new(File::open(path)?);

        let header: VaultHeader = bincode::deserialize_from(&mut reader)?;
        let nonce: [u8; NONCE_SIZE] = bincode::deserialize_from(&mut reader)?;
        let mut ciphertext_bytes = Vec::new();
        let x = reader.read_to_end(&mut ciphertext_bytes).unwrap();
        println!("Got {} bytes", x);

        println!("Header: {:#?}", header);
        println!("Nonce: {:#?}", nonce);
        println!("Ciphertext: {:#?}", ciphertext_bytes);

        let (derived_key, _) = derive_encryption_key(master_password, Some(header.salt));
        let internal_key = decrypt_ciphertext_of_size(
            &header.master_password_key,
            derived_key,
            header.master_password_nonce,
        )
        .unwrap();

        let decrypted_bytes = decrypt_ciphertext(&ciphertext_bytes, internal_key, nonce).unwrap();
        let vault_entries: HashMap<String, VaultEntry> = bincode::deserialize(&decrypted_bytes).unwrap();
        println!("Decrypt: {:#?}", vault_entries);

        Ok(Self {
            header,
            path: path.into(),
            internal_key,
            vault_entries,
        })
    }

    fn add_vault_entry(&mut self, entry_title: String, vault_entry: VaultEntry) {
        self.vault_entries.insert(entry_title, vault_entry);
    }

    pub fn write(&self) -> Result<(), Box<ErrorKind>> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend(bincode::serialize(&self.header)?);

        let entries_bytes = bincode::serialize(&self.vault_entries)?;
        let (nonce, ciphertext) = encrypt_plaintext(&entries_bytes, self.internal_key).unwrap();
        bytes.extend(nonce);
        bytes.extend(ciphertext);
        fs::write(&self.path, bytes)?;
        Ok(())
    }
}

#[test]
fn test_read() {
    let _ = Vault::read(Path::new("/home/loucas/.config/com.spartankey/vault"), "password").unwrap();
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VaultEntry {
    username: String,
    password: String,
    url: String,
}

#[tauri::command]
/// **SHOULD ONLY BE CALLED FROM WEBVIEW** <br>
/// Creates a new vault with the specified `vault_name`, `vault_path` which will be encrypted by the `master_password`
pub fn create_new_vault(
    vault_name: String,
    vault_path: String,
    master_password: String,
    app_handle: tauri::AppHandle<tauri::Wry>,
) {
    println!("vault_name: {}", vault_name);
    println!("vault_path: {}", vault_path);
    println!("master_password: {}", master_password);

    // Generate an obnoxious string for the encryption key.
    let internal_master_key = derive_encryption_key(&generate_password(KEY_SIZE), None).0;

    // Now derive an encryption key from the master password
    let (master_password_key, master_password_key_salt) =
        derive_encryption_key(&master_password, None);

    // And use the encryption key derived from the master_password to encrypt the internal_master_key
    let (mp_encrypted_internal_master_key_nonce, mp_encrypted_internal_master_key) =
        encrypt_plaintext(&internal_master_key, master_password_key).unwrap();

    // TODO: Write the recovery key to a file (maybe noeof?)
    // Derive a random encryption key, this is the recovery key
    let (recovery_key, _) = derive_encryption_key(&generate_password(KEY_SIZE), None);

    // Encrypt the internal master key again, but this time using the randomly derived encryption key.
    let (rk_encrypted_internal_master_key_nonce, rk_encrypted_internal_master_key) =
        encrypt_plaintext(&internal_master_key, recovery_key).unwrap();

    // Get the vault manager
    let vault_manager_state: tauri::State<VaultManagerState> = app_handle
        .try_state()
        .expect("`VaultManager` should already be managed");
    let mut vault_manager = vault_manager_state.0.lock().unwrap();

    let path = Path::new(&vault_path);
    println!("Creating vault at path: {}", vault_path);
    
    // Create a new vault, add it to the vault manager and activate it
    let vault = Vault::new(
        path,
        master_password_key_salt,
        mp_encrypted_internal_master_key_nonce,
        rk_encrypted_internal_master_key_nonce,
        internal_master_key,
        mp_encrypted_internal_master_key,
        rk_encrypted_internal_master_key,
    );
    // Write the vault to the vault path
    vault.write().unwrap();

    vault_manager.add_and_activate_vault(&vault_name, vault);

    // Add the vault mapping to the config
    let config_state: tauri::State<ConfigState> = app_handle.state();
    config_state.add_vault(&vault_name, path);
    config_state.write().expect("Error writing config to file.");
    println!("Done!");
}

#[tauri::command]
/// **SHOULD ONLY BE CALLED FROM WEBVIEW** <br>
/// Adds a new entry to the currently active vault
pub fn add_entry(
    url: String,
    username: String,
    password: String,
    app_handle: tauri::AppHandle<tauri::Wry>,
) {
    // Get the vault manager state and add a vault entry to it
    let vault_manager_state: tauri::State<VaultManagerState> = app_handle
        .try_state()
        .expect("`VaultManager` should already be managed");
    let mut vault_manager = vault_manager_state.0.lock().unwrap();

    if let Some(vault) = vault_manager.get_active_vault() {
        vault.add_vault_entry(
            url.clone(),
            VaultEntry {
                username,
                password,
                url,
            },
        );
        vault.write().unwrap();
    }
}

#[tauri::command]
/// **SHOULD ONLY BE CALLED FROM WEBVIEW** <br>
/// Returns all the entries in the currently active vault
pub fn get_active_vault_entries(app_handle: tauri::AppHandle<tauri::Wry>) -> Vec<VaultEntry> {
    let vault_manager_state: tauri::State<VaultManagerState> = app_handle
        .try_state()
        .expect("`VaultManager` should already be managed");
    let mut vault_manager = vault_manager_state.0.lock().unwrap();

    if let Some(vault) = vault_manager.get_active_vault() {
        return vault.vault_entries.values().map(|entry| entry.clone()).collect();
    }
    Vec::new()
}

#[tauri::command]
/// **SHOULD ONLY BE CALLED FROM WEBVIEW** <br>
/// Returns a vec of all the known vault names.
pub fn get_vaults(app_handle: tauri::AppHandle<tauri::Wry>) -> Vec<String> {
    let config_state: tauri::State<ConfigState> = app_handle.state();
    let config = config_state.state.lock().unwrap();
    config.get_vault_names()
}

#[tauri::command]
/// **SHOULD ONLY BE CALLED FROM WEBVIEW** <br>
/// Tries to read a vault called `name` from disk, decrypt it, and set it to the active vault.
pub fn open_vault(name: String, password: String, app_handle: tauri::AppHandle<tauri::Wry>) {
    let vault_manager_state: tauri::State<VaultManagerState> = app_handle
        .try_state()
        .expect("`VaultManager` should already be managed");
    let mut vault_manager = vault_manager_state.0.lock().unwrap();

    let config_state: tauri::State<ConfigState> = app_handle.state();
    let config = config_state.state.lock().unwrap();
    // TODO: First check if the vault manager already has the vault and retrieve it from there
    // TODO: Otherwise we'll read the vault from disk.
    let path = config
        .get_path(&name)
        .expect(&format!("No vault entry for {}", name));
    println!("Opening: {}", name);
    let vault = Vault::read(path.as_path(), &password).unwrap();
    vault_manager.add_and_activate_vault(&name, vault);
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
fn generate_password(password_size: usize) -> String {
    let lowercase_chars = "abcdefghijklmnopqrstuvwxyz".as_bytes();
    let uppercase_chars = "ABCDEFGHIJKLMNOPQRSTYVWXYZ".as_bytes();
    let special_chars = "!@#$%^&*".as_bytes();
    let digits = "01234567890".as_bytes();

    // Randomly generate the number of each type of character.
    let mut rng = rand::thread_rng();
    let num_lowercase = rng.gen_range(1..password_size - 3);
    let num_uppercase = rng.gen_range(1..password_size - num_lowercase - 2);
    let num_digits = rng.gen_range(1..password_size - num_lowercase - num_uppercase - 1);
    let num_special = password_size - (num_lowercase + num_uppercase + num_digits);

    // Again randomly generate which characters in that set of characters to use.
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

    // Not possible to be a non-utf8 character
    String::from_utf8(bytes).unwrap()
}

/// Derive the encryption key from the master password and user's email address.
/// The email is treated as an "add" to make the hash tougher to break and is concatenated with the
/// master_password to form the "password" which the encryption key is derived from.
///
/// Automatically generate a 128-bit salt and perform SHA512 + PBKDF2 + HMAC with 100k iterations
///
/// Returns a tuple of `(256-bit encryption key, salt)`
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

/// Takes the bytes to encrypt and the encryption key and creates the ciphertext
/// A 96-bit nonce is randomly generated and used during the AES256-GCM process.
///
/// The generated ciphertext will have a 16 byte authentication tag appended to it.
///
/// Returns a tuple of (nonce, ciphertext)
fn encrypt_plaintext(
    bytes: &[u8],
    key_bytes: [u8; KEY_SIZE],
) -> EncryptionResult<([u8; NONCE_SIZE], Vec<u8>)> {
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)?;

    let nonce_bytes = rand::thread_rng().gen::<[u8; NONCE_SIZE]>();
    let nonce = Nonce::from_slice(&nonce_bytes); // 96-bits; unique per message

    let ciphertext = cipher.encrypt(nonce, bytes)?;
    Ok((nonce_bytes, ciphertext))
}

/// Accepts the ciphertext, encryption key, and nonce and decrypts the ciphertext
///
/// Returns the decrypted ciphertext or an error.
/// Error would indicate that either the key is wrong or the ciphertext was changed.
fn decrypt_ciphertext(
    ciphertext: &[u8],
    key_bytes: [u8; KEY_SIZE],
    nonce_bytes: [u8; NONCE_SIZE],
) -> EncryptionResult<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)?;
    let nonce: &GenericArray<u8, typenum::U12> = Nonce::from_slice(&nonce_bytes);

    hex_print("ciphertext", &ciphertext);

    Ok(cipher.decrypt(nonce, ciphertext)?)
}

/// Accepts the ciphertext, encryption key, and nonce and decrypts the ciphertext
///
/// Returns the decrypted ciphertext or an error.
/// Error would indicate that either the key is wrong or the ciphertext was changed.
fn decrypt_ciphertext_of_size<const N: usize>(
    ciphertext: &[u8],
    key_bytes: [u8; KEY_SIZE],
    nonce_bytes: [u8; NONCE_SIZE],
) -> EncryptionResult<[u8; N]> {
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)?;
    let nonce: &GenericArray<u8, typenum::U12> = Nonce::from_slice(&nonce_bytes);

    hex_print("ciphertext", &ciphertext);

    let text = cipher.decrypt(nonce, &*ciphertext.to_vec())?;
    Ok(text.try_into().unwrap_or_else(|v: Vec<u8>| {
        panic!("Expected a Vec of length {} but it was {}", N, v.len())
    }))
}

fn hex_print(prefix: &str, bytes: &[u8]) {
    print!("{}: ", prefix);
    for byte in bytes {
        print!("{:02X}", byte);
    }
    println!();
}
