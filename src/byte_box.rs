use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

/// The core struct of the library.\
/// Contains the path to the directory where the data is stored and implements many useful functions.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Resource))]
pub struct ByteBox {
    /// The path to the directory where the data is stored.
    path: PathBuf,
}

impl ByteBox {
    /// Creates a new `ByteBox` instance storing data in the specified directory.
    #[inline(always)]
    pub fn new(path: impl AsRef<Path>) -> Option<Self> {
        fs::create_dir_all(&path).ok()?;
        Some(Self {
            path: path.as_ref().to_path_buf(),
        })
    }

    /// Creates a new `ByteBox` instance with automatically built data directory based on the given app name.
    #[cfg(feature = "path")]
    #[inline(always)]
    pub fn default(app_name: impl AsRef<Path>) -> Option<Self> {
        Self::new(crate::path::build_app_path(app_name)?)
    }

    /// Returns the path to the directory where the data is stored.
    #[inline(always)]
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Writes the given value to the file with the given identifier.\
    /// The file will be named `<identifier>.msgpack`.
    pub fn set(&self, identifier: impl ToString, val: &impl Serialize) -> Option<()> {
        let mut path = self.path.clone();
        path.push(identifier.to_string());
        path.set_extension("msgpack");

        rmp_serde::encode::write(&mut File::create(&path).ok()?, val).ok()?;
        Some(())
    }

    /// Reads the value from the file with the given identifier.\
    /// The file should be named `<identifier>.msgpack`.
    pub fn get<V: for<'a> Deserialize<'a>>(&self, identifier: impl ToString) -> Option<V> {
        let mut path = self.path.clone();
        path.push(identifier.to_string());
        path.set_extension("msgpack");

        let file = File::open(&path).ok()?;
        rmp_serde::decode::from_read(file).ok()
    }
}
