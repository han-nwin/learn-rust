fn main() {
    // 1. Creating and displaying
    let mut greeting = String::from("Hello");
    greeting.push_str(", ");
    println!("{}", greeting); // Hello,

    // 2. Using string formatting
    let name = "World";
    let msg = format!("Hello, {}!", name);

    // 3. Converting UTF-8 data
    let bytes = b"Hello";
    let s = String::from_utf8(bytes.to_vec()).unwrap();
    println!("{}", s);

    // 4. Accessing UTF-8 chars
    let mut chars = "日本語".chars();
    for ch in chars {
        println!("{}", ch);
    }
}
