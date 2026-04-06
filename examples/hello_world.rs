use std::path::PathBuf;

use bytebox::ByteBox;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyBox {
    special_number: i32,
    greet: bool,
    name: String,
    age: i32,
}

impl ByteBox<true> for MyBox {
    fn path(&self) -> PathBuf {
        PathBuf::from("my_box.bin")
    }
}

fn main() {
    let my_box = MyBox {
        special_number: 12,
        greet: true,
        name: "John".to_string(),
        age: 19,
    };

    my_box.save().expect("Could not save ByteBox");

    let mut loaded_box = MyBox {
        special_number: 0,
        greet: false,
        name: String::new(),
        age: 0,
    };

    loaded_box.load().expect("Could not load ByteBox");

    if loaded_box.greet {
        println!(
            "Hello, {}! You are {} years old.",
            loaded_box.name, loaded_box.age
        );
    }

    println!("The special number is {}", loaded_box.special_number);

    loaded_box.delete().unwrap();
}
