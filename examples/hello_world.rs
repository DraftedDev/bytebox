use bytebox::{byte_box::ByteBox, Deserialize, Serialize};

static APP_NAME: &str = "hello_world";
static BOX_NAME: &str = "my awesome box";

#[derive(Serialize, Deserialize)]
struct MyBox {
    special_number: i32,
    greet: bool,
    name: String,
    age: i32,
}

#[cfg(feature = "default")]
fn main() {
    let bytebox = ByteBox::default(APP_NAME).expect("Could not create ByteBox");

    bytebox
        .set(
            BOX_NAME,
            &MyBox {
                special_number: 12,
                greet: true,
                name: "John".to_string(),
                age: 19,
            },
        )
        .expect("Could not write to ByteBox");

    let my_box = bytebox
        .get::<MyBox>(BOX_NAME)
        .expect("Could not read from ByteBox");

    if my_box.greet {
        println!("Hello, {}! You are {} years old.", my_box.name, my_box.age);
    }

    println!("The special number is {}", my_box.special_number);
}
