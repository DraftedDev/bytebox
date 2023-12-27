# ByteBox 📦

ByteBox is an easy and performant data storage solution based on MessagePack. It provides a simple interface for storing
and retrieving data in a compact format, making it efficient for various applications.

## Features ✨

- **Compact Storage:** ByteBox uses the MessagePack format, ensuring a compact representation of your data.
- **Easy Integration:** Simple API for storing and retrieving data with just a few method calls.
- **Efficient Serialization:** Optimized serialization and deserialization using the rmp_serde library.

## Getting Started 🚀

To use ByteBox in your Rust project, run: ``cargo add bytebox`` or just add the latest version to your `Cargo.toml`.

## Examples 📝
- [Hello World](examples/hello_world.rs)

## Bevy Integration
You can easily use ByteBox in your Bevy app by adding the `bevy` feature and registering the `ByteboxPlugin` plugin.

Add your boxes by calling `ByteboxPlugin::new().with(my_box)` and access your boxes via the `Res<ByteBox>` type in your systems.
