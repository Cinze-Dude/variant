use variant::variant::Variant::{self};

fn main() {
    println!("Hello, world!");

    let var: Variant<&str, i32> = Variant::from_any(8).unwrap();
}
