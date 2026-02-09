use soc_gs::datatypes::Data;

fn main() {
    println!("Hello, world!");

    let d1 = Data {
        id: 1,
        name: String::from("Test"),
    };

    println!("{d1}");
}
