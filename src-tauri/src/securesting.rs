use std::{iter::zip, fmt::Display};

struct SecureString {
    str_bytes: Vec<u8>,
    key: Vec<u8>,
}

impl SecureString {
    pub fn deobfuscate(&self) -> String {
        let bytes = zip(&self.str_bytes, &self.key).map(|(str_byte, key_byte)| str_byte ^ key_byte).collect();
        String::from_utf8(bytes).unwrap()
    }

    fn generate_key(length: usize) -> Vec<u8> {
        (0..length).map(|_| rand::random::<u8>()).collect()
    }

    fn xor_cihper(bytes: Vec<u8>, key: &[u8]) -> Vec<u8> {
        zip(bytes, key).map(|(str_byte, key_byte)| str_byte ^ key_byte).collect()
    }
}

impl From<Vec<u8>> for SecureString {
    fn from(value: Vec<u8>) -> Self {
        let key = Self::generate_key(value.len());
        let str_bytes = Self::xor_cihper(value, &key);
        Self {
            str_bytes,
            key
        }
    }
}

impl<const N: usize> From<&[u8; N]> for SecureString {
    fn from(value: &[u8; N]) -> Self {
        Self::from(Into::<Vec<u8>>::into(*value))
    }
}

impl From<String> for SecureString {
    fn from(value: String) -> Self {
        let bytes = value.into_bytes();
        Self::from(bytes)
    }
}

impl Display for SecureString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("**SECURESTRING**")
    }
}

#[test]
fn test_secure_string() {
    let s = SecureString::from(b"Testing");
    println!("Testing: {}", s);
    println!("Obfuscated: {:#?}", String::from_utf8_lossy(&s.str_bytes));
    println!("Deobfuscated: {:#?}", s.deobfuscate());
}