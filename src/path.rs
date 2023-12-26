use std::path::{Path, PathBuf};

/// Builds the path to the application data directory.
pub fn build_app_path(ident: impl AsRef<Path>) -> Option<PathBuf> {
    Option::from(dirs::data_dir()?.join(ident))
}
