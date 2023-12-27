pub use serde::Deserialize;
pub use serde::Serialize;

/// Contains the [ByteBox](byte_box::ByteBox) struct.\
/// The core of the library.
pub mod byte_box;

/// Contains useful functions for working with common paths.\
/// Uses [dirs] internally.
#[cfg(feature = "path")]
pub mod path;

/// Contains the [ByteboxPlugin](bevy::ByteboxPlugin) plugin and other stuff.
#[cfg(feature = "bevy")]
pub mod bevy;

#[cfg(test)]
mod tests;
