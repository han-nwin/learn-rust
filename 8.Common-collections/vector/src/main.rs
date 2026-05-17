fn main() {
    let mut v: Vec<i32> = Vec::new();

    // or use vec! macro
    let v2 = vec![1, 2, 3];

    v.push(2);
    v.push(3);

    println!("{:?}", v);
    println!("{:?}", v2);

    // Access elements
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let does_not_exist = &v[100]; // THIS WILL PANIC
    let does_not_exist = v.get(100); // THIS RETURNS NONE
}
