use std::fmt::{Display, Formatter};

/// An error that can occur while operating on a bytebox.
#[derive(Debug)]
pub enum Error {
    /// An I/O error.
    Io(std::io::Error),
    /// A Serde error.
    Serde(bitcode::Error),
    /// A cipher error.
    Cipher(chacha20poly1305::Error),
    /// A keyring error.
    #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
    KeyRing(keyring::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Serde(e) => write!(f, "Serde error: {}", e),
            Error::Cipher(e) => write!(f, "Cipher error: {}", e),
            #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
            Error::KeyRing(e) => write!(f, "KeyRing error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<bitcode::Error> for Error {
    fn from(e: bitcode::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<chacha20poly1305::Error> for Error {
    fn from(e: chacha20poly1305::Error) -> Self {
        Error::Cipher(e)
    }
}

#[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
impl From<keyring::Error> for Error {
    fn from(e: keyring::Error) -> Self {
        Error::KeyRing(e)
    }
}
