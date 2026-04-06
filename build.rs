fn main() {
    if cfg!(all(feature = "tokio", feature = "async-fs")) {
        panic!(
            "Both 'tokio' and 'async-fs' features are enabled. Please chose just one async API!"
        );
    }

    cfg_aliases::cfg_aliases! {
        use_keyring: {
            any(
                target_os = "linux",
                target_os = "macos",
                target_os = "ios",
                target_os = "windows",
                target_os = "freebsd",
                target_os = "openbsd"
            )
        },
        async_api: { any(feature = "tokio", feature = "async-fs") }
    }
}
