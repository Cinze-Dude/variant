use variant::variant::Variant::{self};

fn main() {
    println!("Hello, world!");

    let var: Variant<&str, i32> = Variant::Second(-65);

    let fold = var.fold(|x| x.len(), |x| x as usize);

    println!("{:?}", fold);
}
