#[test]
fn assert_path() {
    let bytebox = crate::byte_box::ByteBox::new("some_location").expect("failed to create bytebox");

    assert_eq!(
        bytebox
            .path()
            .canonicalize()
            .expect("failed to canonicalize bytebox path"),
        std::env::current_dir()
            .expect("failed to get current dir")
            .join("some_location")
            .canonicalize()
            .expect("failed to canonicalize path")
    );

    bytebox.delete();
}

#[cfg(feature = "path")]
#[test]
fn assert_app_path() {
    let bytebox = crate::byte_box::ByteBox::default("some_app").expect("failed to create bytebox");

    assert_eq!(
        bytebox
            .path()
            .canonicalize()
            .expect("failed to canonicalize bytebox path"),
        crate::path::build_app_path("some_app")
            .expect("failed to build app path")
            .canonicalize()
            .expect("failed to canonicalize app path")
    );

    bytebox.delete();
}

#[cfg(feature = "bevy")]
#[test]
fn test_bevy_plugin() {
    use bevy::prelude::*;

    let bytebox = crate::byte_box::ByteBox::new("some_location").expect("failed to create bytebox");

    fn do_stuff(bytebox: Res<crate::byte_box::ByteBox>) {
        bytebox.delete();
    }

    App::new()
        .add_plugins((crate::bevy::ByteboxPlugin::new().with(bytebox),))
        .add_systems(Startup, do_stuff)
        .set_runner(|mut app| {
            app.update();
            app.cleanup();
        })
        .run();
}
