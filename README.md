# ByteBox 📦

**ByteBox is an easy and performant data storage solution. It provides a way to read and write data, optionally in a secure format.**

## Features ✨

- **Fast** - Serialization uses [bitcode](https://crates.io/crates/bitcode) for maximum decoding and encoding performance.
- **Ease-of-use** - Use [serde](https://crates.io/crates/serde) and a super-easy API to store and retrieve your data.
- **Secure** - Secure your data using the XChaCha20Poly1305 algorithm without the hassle of storing and retrieving encryption keys manually.

## Supported Platforms
- [x] Desktop: Windows, MacOS, Linux, FreeBSD, OpenBSD.
- [x] Mobile: iOS (Android work-in-progress).
- [ ] Web

## Getting Started 🚀

To use ByteBox in your Rust project, run: ``cargo add bytebox`` or just add the latest version to your `Cargo.toml`.

## Examples 📝
- [Hello World](examples/hello_world.rs)
