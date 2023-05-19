use std::{
    collections::HashMap,
    fs::{self},
    io,
    path::{Path, PathBuf},
    sync::Mutex,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    vaults: HashMap<String, PathBuf>
}

impl Config {
    fn add_vault(&mut self, vault_name: &str, vault_path: &Path) {
        self.vaults.insert(vault_name.into(), vault_path.into());
    }

    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

pub struct ConfigState {
    path: PathBuf,
    pub state: Mutex<Config>,
}

impl ConfigState {
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

        Self {
            path,
            state: Mutex::new(config),
        }
    }

    pub fn write(&self) -> io::Result<()> {
        let bytes = &self.state.lock().unwrap().serialize();
        fs::write(&self.path, bytes)?;
        Ok(())
    }

    pub fn add_vault(&self, vault_name: &str, vault_path: &Path) {
        let config = &mut self.state.lock().unwrap();
        config.add_vault(vault_name, vault_path);
    }

}