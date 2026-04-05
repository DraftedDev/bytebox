#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    secure::{KeyStore, PlatformKeyStore},
};

/// Contains the [error::Error] type.
pub mod error;

/// Contains functionality for secure storage and encryption of data.
pub mod secure;

/// A global data store that can optionally use encryption for secure storage.
///
/// This is a variant of [ByteBox] that can always be constructed without a `self`.
///
/// The `SECURE` const parameter determines whether encryption is used.
pub trait GlobalByteBox<const SECURE: bool>: Serialize + for<'de> Deserialize<'de> {
    /// Returns the path to the bytebox file.
    fn path() -> PathBuf;

    /// Saves the bytebox to disk.
    ///
    /// This will create a new file if one does not already exist.
    fn save(&self) -> Result<(), Error> {
        let path = Self::path();
        let bytes = self.encode()?;

        std::fs::write(&path, bytes)?;

        Ok(())
    }

    /// Loads the bytebox from disk.
    fn load() -> Result<Self, Error>
    where
        Self: Sized,
    {
        let path = Self::path();
        let bytes = std::fs::read(&path)?;

        Self::decode(&bytes)
    }

    /// Encodes the bytebox into a byte vector.
    ///
    /// Also encrypts the content if `SECURE` is `true`.
    fn encode(&self) -> Result<Vec<u8>, Error> {
        if SECURE {
            let name = format!("{}", Self::path().display());

            let bytes = bitcode::serialize(self)?;
            let key = PlatformKeyStore::get_key_or_generate(&name)?;

            secure::encrypt(&bytes, key)
        } else {
            Ok(bitcode::serialize(self)?)
        }
    }

    /// Decodes a byte vector into a bytebox.
    ///
    /// Also decrypts the content if `SECURE` is `true`.
    fn decode(bytes: &[u8]) -> Result<Self, Error> {
        if SECURE {
            let name = Self::path().to_string_lossy().to_string();
            let key = PlatformKeyStore::get_key_or_generate(&name)?;
            let decrypted = secure::decrypt(bytes.to_vec(), key)?;

            Ok(bitcode::deserialize(&decrypted)?)
        } else {
            Ok(bitcode::deserialize(bytes)?)
        }
    }
}

/// A data store that can optionally use encryption for secure storage.
///
/// Unlike [GlobalByteBox], this trait is not global and requires a `self` to be loaded.
///
/// The `SECURE` const parameter determines whether encryption is used.
pub trait ByteBox<const SECURE: bool>: Serialize + for<'de> Deserialize<'de> {
    /// Returns the path to the bytebox file.
    fn path(&self) -> PathBuf;

    /// Saves the bytebox to disk.
    ///
    /// This will create a new file, if one does not already exist.
    fn save(&self) -> Result<(), Error> {
        let path = self.path();
        let bytes = self.encode()?;

        std::fs::write(&path, bytes)?;

        Ok(())
    }

    /// Loads the bytebox from disk.
    fn load(&mut self) -> Result<(), Error> {
        let path = self.path();
        let bytes = std::fs::read(&path)?;

        *self = self.decode(&bytes)?;

        Ok(())
    }

    /// Encodes the bytebox into a byte vector.
    ///
    /// Also encrypts the byte vector if `SECURE` is `true`.
    fn encode(&self) -> Result<Vec<u8>, Error> {
        if SECURE {
            let name = format!("{}", self.path().display());
            let bytes = bitcode::serialize(self)?;
            let key = PlatformKeyStore::get_key_or_generate(&name)?;

            secure::encrypt(&bytes, key)
        } else {
            Ok(bitcode::serialize(self)?)
        }
    }

    /// Decodes a byte vector into the bytebox.
    ///
    /// Also decrypts the byte vector if `SECURE` is `true`.
    fn decode(&self, bytes: &[u8]) -> Result<Self, Error> {
        if SECURE {
            let name = format!("{}", self.path().display());

            let key = PlatformKeyStore::get_key_or_generate(&name)?;
            let decrypted = secure::decrypt(bytes.to_vec(), key)?;

            Ok(bitcode::deserialize(&decrypted)?)
        } else {
            Ok(bitcode::deserialize(bytes)?)
        }
    }
}
