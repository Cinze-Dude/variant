use variant::variant::Variant::{self};

fn main() {
    println!("Hello, world!");

    let var: Variant<&str, &str> = Variant::from_any("56").unwrap();

    println!("{:?}", var.collapse())
}
