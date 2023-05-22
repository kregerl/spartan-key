use std::{
    collections::HashMap,
    fs::{self},
    io,
    path::{Path, PathBuf},
    sync::Mutex,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
/// Structure for managing the mapping between vault names and where they're stored on disk.
pub struct Config {
    vaults: HashMap<String, PathBuf>
}

impl Config {
    /// Add a vault to the config file's map of known vaults.
    fn add_vault(&mut self, vault_name: &str, vault_path: &Path) {
        self.vaults.insert(vault_name.into(), vault_path.into());
    }

    /// Serialize the config to bytes using bincode
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn get_vault_names(&self) -> Vec<String> {
        self.vaults.keys().map(|key| key.clone()).collect()
    }

    pub fn get_vaults(&self) -> &HashMap<String, PathBuf> {
        &self.vaults
    }

    pub fn get_path(&self, name: &str) -> Option<&PathBuf> {
        self.vaults.get(name)
    }
}

/// Wrapper for [Config] allowing modification from seperate threads(or tauri commands) with a mutex
pub struct ConfigState {
    path: PathBuf,
    pub state: Mutex<Config>,
}

impl ConfigState {
    /// Create a new config state, if there already exists a `config` binary file in the app_dir then 
    /// that will be loaded.
    /// 
    /// If there is no config file, and empty one is created.
    pub fn new(app_dir: &Path) -> Self {
        let path = app_dir.join("config");
        // If the path already exists, read the config file otherwise create empty one.
        let config = if path.exists() {
            let bytes = fs::read(&path).unwrap();
            bincode::deserialize(&bytes)
                .expect("Could not deserialize config file, is it formatted incorrectly?")
        } else {
            Config::default()
        };

        println!("Loaded config: {:#?}", config);

        Self {
            path,
            state: Mutex::new(config),
        }
    }

    /// Write the config state to the `config` file
    pub fn write(&self) -> io::Result<()> {
        let bytes = &self.state.lock().unwrap().serialize();
        fs::write(&self.path, bytes)?;
        Ok(())
    }

    /// Add a vault to the `state`
    pub fn add_vault(&self, vault_name: &str, vault_path: &Path) {
        let config = &mut self.state.lock().unwrap();
        config.add_vault(vault_name, vault_path);
    }

}