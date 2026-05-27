fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// the 2 things above can be combined in 1 function that operate on generic type
// We read this definition as “The function largest is generic over some type T.”
// This function has one parameter named list, which is a slice of values of type T.
// The largest function will return a reference to a value of the same type T.
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("the largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("the largest char is {result}");

    let result = largest(&number_list);
    println!("the largest number is {result}");

    let result = largest(&char_list);
    println!("the largest char is {result}");
}
