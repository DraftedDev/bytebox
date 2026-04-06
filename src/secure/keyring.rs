use keyring::Entry;

use crate::secure::namespace;

/// A key store that uses [keyring].
pub struct KeyStore;

impl super::KeyStore for KeyStore {
    fn get_key(&self, name: &str) -> Result<Option<Vec<u8>>, crate::error::Error> {
        let entry = Entry::new(namespace()?.as_str(), name)?;

        match entry.get_secret() {
            Ok(secret) => Ok(Some(secret)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(crate::error::Error::KeyRing(e)),
        }
    }

    fn set_key(&self, name: &str, key: Vec<u8>) -> Result<(), crate::error::Error> {
        let entry = Entry::new(namespace()?.as_str(), name)?;

        entry.set_secret(&key)?;

        Ok(())
    }
}
