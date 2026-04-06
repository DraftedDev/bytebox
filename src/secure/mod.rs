use std::sync::OnceLock;

use chacha20poly1305::{
    aead::{AeadMut, Key, Nonce, OsRng},
    AeadCore, KeyInit, XChaCha20Poly1305,
};

use crate::error::Error;

/// A [KeyStore] implementation that uses [keyring] on linux, macOS, and Windows.
#[cfg(use_keyring)]
pub mod keyring;

/// The default [KeyStore] implementation for this platform.
#[cfg(use_keyring)]
pub type PlatformKeyStore = keyring::KeyStore;

const NONCE_SIZE: usize = 24;

/// The global [KeyStore] instance.
///
/// Normally initialized with the default implementation on first access.
pub static KEY_STORE: OnceLock<Box<dyn KeyStore>> = OnceLock::new();

/// Returns the set [KeyStore] implementation, initializing it with the default if necessary.
pub fn get_key_store<'a>() -> &'a dyn KeyStore {
    KEY_STORE.get_or_init(default_key_store)
}

/// Returns the default [KeyStore] implementation for this platform.
pub fn default_key_store() -> Box<dyn KeyStore> {
    #[cfg(use_keyring)]
    {
        Box::new(keyring::KeyStore)
    }

    #[cfg(not(use_keyring))]
    {
        panic!("Platform not supported!");
    }
}

/// Encrypts the given data using the provided key.
pub fn encrypt(data: &[u8], key: Vec<u8>) -> Result<Vec<u8>, Error> {
    let key = Key::<XChaCha20Poly1305>::from_iter(key);
    let mut cipher = XChaCha20Poly1305::new(&key);
    let nonce = generate_nonce();

    let mut encrypted = cipher.encrypt(&nonce, data)?;
    encrypted.reserve(nonce.len());
    encrypted.extend(nonce.as_slice());

    Ok(encrypted)
}

/// Decrypts the given data using the provided key.
pub fn decrypt(mut data: Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>, Error> {
    let key = Key::<XChaCha20Poly1305>::from_iter(key);
    let mut cipher = XChaCha20Poly1305::new(&key);
    let nonce_data = data
        .drain(data.len() - NONCE_SIZE..data.len())
        .collect::<Vec<_>>();
    let nonce = Nonce::<XChaCha20Poly1305>::from_slice(nonce_data.as_slice());

    Ok(cipher.decrypt(nonce, data.as_slice())?)
}

/// Generates a new random key via the XChaCha20Poly1305 algorithm.
pub fn generate_key() -> Key<XChaCha20Poly1305> {
    XChaCha20Poly1305::generate_key(OsRng)
}

/// Generates a new random nonce via the XChaCha20Poly1305 algorithm.
pub fn generate_nonce() -> Nonce<XChaCha20Poly1305> {
    XChaCha20Poly1305::generate_nonce(OsRng)
}

/// Returns the namespace for the current process.
///
/// The namespace is used to store keys in a process-specific manner.
pub fn namespace() -> Result<String, Error> {
    let process = std::env::current_exe()?;

    Ok(process
        .file_name()
        .expect("Failed to get file name")
        .to_string_lossy()
        .to_string())
}

/// A trait for storing and retrieving keys.
pub trait KeyStore: Send + Sync + 'static {
    /// Returns the key with the given name, generating a new one if none exists.
    fn get_key_or_generate(&self, name: &str) -> Result<Vec<u8>, Error> {
        let key = self.get_key(name)?;

        if let Some(key) = key {
            Ok(key)
        } else {
            let key = generate_key();
            self.set_key(name, key.to_vec())?;
            Ok(key.to_vec())
        }
    }

    /// Returns the key with the given name, if one exists.
    fn get_key(&self, name: &str) -> Result<Option<Vec<u8>>, Error>;

    /// Sets the key with the given name.
    fn set_key(&self, name: &str, key: Vec<u8>) -> Result<(), Error>;
}

impl KeyStore for Box<dyn KeyStore> {
    fn get_key(&self, name: &str) -> Result<Option<Vec<u8>>, Error> {
        (**self).get_key(name)
    }

    fn set_key(&self, name: &str, key: Vec<u8>) -> Result<(), Error> {
        (**self).set_key(name, key)
    }
}
