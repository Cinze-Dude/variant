use variant::variant::Variant::{self, First};

fn main() {
    println!("Hello, world!");

    let var: Variant<&str, i32> = First("variant");
}
