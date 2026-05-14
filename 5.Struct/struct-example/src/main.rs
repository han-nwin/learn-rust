use std::mem::{size_of, size_of_val};

#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64,
}

fn main() {
    let scale = 2;
    let rec = Rectangle {
        width: dbg!(30 * scale), // dbg return the ownership back to width
        height: 50,
    };

    println!("rec is {rec:?}");
    dbg!(&rec); // we don't want dbg to take ownership of rec here

    println!(
        "Rectangle type size (2 u64): {} bytes",
        size_of::<Rectangle>()
    );
    println!("rec value size: {} bytes", size_of_val(&rec));
    println!(
        "&Rectangle size (address size): {} bytes",
        size_of::<&Rectangle>()
    );
    println!("u64 size (size of u64): {} bytes", size_of::<u64>());

    println!("area by reference: {}", area_ref(&rec));
    println!("area by value: {}", area_val(rec));
}

// this should use less memeory
fn area_ref(rec: &Rectangle) -> u64 {
    rec.width * rec.height
}

fn area_val(rec: Rectangle) -> u64 {
    rec.width * rec.height
}
