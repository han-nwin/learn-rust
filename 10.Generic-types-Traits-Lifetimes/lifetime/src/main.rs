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
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());

        println!("The longest string is {result}"); // this works
    }
    // println!("The longest string is {result}"); // this won't. since string2 already out of scope
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Rust doesn't know the lifetime of returned &str
    // so need to use 'a to tell
    if x.len() > y.len() { x } else { y }
} // NOTE: 'a is just a generic for reference (like T) -> hence the <'a>
//

//Relationship
fn longest_2<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
// return lifetime doesn't know about y -> if y go out of scope it'll fail
