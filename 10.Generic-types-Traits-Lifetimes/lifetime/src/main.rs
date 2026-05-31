// The main aim of lifetime is to prevent dangling reference
fn main0() {
    let r;

    {
        let x = 5;
        r = &x; // x doesn't live long enough
    }
    println!("r: {r}"); // x scope already end by now
}

#[rustfmt::skip]
// visual
fn main1() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   //          |
} // ---------+
// a reference a memory of b but b dies before a -> ERROR

#[rustfmt::skip]
fn main2() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
} // ----------+
// here a reference a memoery of b but b dies after a => ACCEPTED

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Rust doesn't know the lifetime of returned &str
    // so need to use 'a to tell
    if x.len() > y.len() { x } else { y }
}
