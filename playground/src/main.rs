fn main() {
    let st = String::from("yoo");
    let mut pt: &str = &st;

    println!("The string: {st}");

    // Actual heap address
    println!("heap address: {:p}", st.as_ptr());
    // Wrapper value on the stack
    println!("wrapper value on the stack: {:p}", pt);
    // Address on the stack of the wrapper
    println!("address of the wrapper on the stack {:p}", &pt);
}
