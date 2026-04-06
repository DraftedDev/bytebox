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
#[cfg_attr(async_api, async_trait::async_trait)]
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

    /// Saves the bytebox to disk asynchronously.
    ///
    /// This will create a new file if one does not already exist.
    #[cfg(async_api)]
    async fn save_async(&self) -> Result<(), Error> {
        let path = Self::path();
        let bytes = self.encode()?;

        #[cfg(feature = "async-fs")]
        async_fs::write(&path, bytes).await?;

        #[cfg(feature = "tokio")]
        tokio::fs::write(&path, bytes).await?;

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

    /// Loads the bytebox from disk asynchronously.
    #[cfg(async_api)]
    async fn load_async() -> Result<Self, Error>
    where
        Self: Sized,
    {
        let path = Self::path();

        #[cfg(feature = "async-fs")]
        let bytes: Vec<u8> = async_fs::read(&path).await?;

        #[cfg(feature = "tokio")]
        let bytes: Vec<u8> = tokio::fs::read(&path).await?;

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

    /// Deletes the bytebox file from disk.
    fn delete() -> Result<(), Error> {
        std::fs::remove_file(Self::path())?;

        Ok(())
    }

    /// Deletes the bytebox file from disk asynchronously.
    #[cfg(async_api)]
    async fn delete_async(&self) -> Result<(), Error> {
        #[cfg(feature = "async-fs")]
        async_fs::remove_file(Self::path()).await?;

        #[cfg(feature = "tokio")]
        tokio::fs::remove_file(Self::path()).await?;

        Ok(())
    }
}

/// A data store that can optionally use encryption for secure storage.
///
/// Unlike [GlobalByteBox], this trait is not global and requires a `self` to be loaded.
///
/// The `SECURE` const parameter determines whether encryption is used.
#[cfg_attr(async_api, async_trait::async_trait)]
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

    /// Loads the bytebox from disk asynchronously.
    #[cfg(async_api)]
    async fn load_async(&mut self) -> Result<(), Error> {
        let path = self.path();

        #[cfg(feature = "async-fs")]
        let bytes: Vec<u8> = async_fs::read(&path).await?;

        #[cfg(feature = "tokio")]
        let bytes: Vec<u8> = tokio::fs::read(&path).await?;

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

    /// Deletes the bytebox file from disk.
    fn delete(&self) -> Result<(), Error> {
        std::fs::remove_file(self.path())?;

        Ok(())
    }

    /// Saves the bytebox to disk asynchronously.
    #[cfg(async_api)]
    async fn save_async(&self) -> Result<(), Error> {
        let path = self.path();
        let bytes = self.encode()?;

        #[cfg(feature = "async-fs")]
        async_fs::write(&path, bytes).await?;

        #[cfg(feature = "tokio")]
        tokio::fs::write(&path, bytes).await?;

        Ok(())
    }

    /// Deletes the bytebox file from disk asynchronously.
    #[cfg(async_api)]
    async fn delete_async(&self) -> Result<(), Error> {
        #[cfg(feature = "async-fs")]
        async_fs::remove_file(self.path()).await?;

        #[cfg(feature = "tokio")]
        tokio::fs::remove_file(self.path()).await?;

        Ok(())
    }
}
