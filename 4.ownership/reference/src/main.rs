// A reference is a a pointer pointing to the address on
// the stack and read its' value (can either be on stack or heap)
// Stack:
//
// s
// ┌────────────────────┐
// │ ptr ─────────────┐ │
// │ len = 5          │ │
// │ cap = 5          │ │
// └────────────────────┘ │
//                        │
// Heap:                  │
// ┌────────────────────┐ │
// │ h e l l o          │◄┘
// └────────────────────┘
//
// r
// ┌────────────────────┐
// │ address of s       │
// └────────────────────┘
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    // let s = String::from("hello");
    // change1(&s);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
// it refers to, the String is not dropped.

// This will throw error if we tryna modify from a referene
// fn change1(some_string: &String) {
//     some_string.push_str(", world");
// }

// Make it a mutable reference then we can modify

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
