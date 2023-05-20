use std::fmt::Display;

use aes_gcm::aead;
use sha2::digest::crypto_common;

pub type EncryptionResult<T> = Result<T, EncryptionError>;

#[derive(Debug)]
pub enum EncryptionError {
    InvalidLength(crypto_common::InvalidLength),
    Aes(aead::Error),
}

impl Display for EncryptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncryptionError::InvalidLength(e) => {
                f.write_fmt(format_args!("EncryptionError: {}", e))
            }
            EncryptionError::Aes(e) => f.write_fmt(format_args!("EncryptionError: {}", e)),
        }
    }
}

impl From<crypto_common::InvalidLength> for EncryptionError {
    fn from(value: crypto_common::InvalidLength) -> Self {
        Self::InvalidLength(value)
    }
}

impl From<aead::Error> for EncryptionError {
    fn from(value: aead::Error) -> Self {
        Self::Aes(value)
    }
}