#[derive(Serialize, Deserialize, Debug, Default)]
struct Vault {
    vault_entries: HashMap<String, VaultEntry>
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct VaultEntry {
    username: String,
    password: String,
    url: String
}