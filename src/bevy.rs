use crate::byte_box::ByteBox;
use bevy::app::{App, Plugin};

/// A simple Plugin to insert ByteBox resources into the bevy app.\
/// Doesn't do anything else. Work in progress.
#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ByteboxPlugin(pub Vec<ByteBox>);

impl ByteboxPlugin {
    /// Creates a new `ByteboxPlugin` instance.
    #[inline(always)]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Adds a new `ByteBox` resource to the plugin.
    #[inline(always)]
    pub fn with(mut self, bytebox: ByteBox) -> Self {
        self.0.push(bytebox);
        self
    }
}

impl Plugin for ByteboxPlugin {
    #[inline(always)]
    fn build(&self, app: &mut App) {
        for bytebox in &self.0 {
            app.insert_resource(bytebox.clone());
        }
    }

    #[inline(always)]
    fn name(&self) -> &str {
        "ByteboxPlugin"
    }
}
