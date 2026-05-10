fn main() {
    // The String Type
    let mut s = String::from("hello"); // create a string from a string literal

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // this will print `hello, world!`
} // this scope is now over, and s is no longer valid. drop() is called
