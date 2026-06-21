fn main() {
    let list = [1, 2, 3];
    let vec_iter = list.iter();

    for val in vec_iter {
        println!("Got {val}");
    }
}

// Iterator trait
//pub trait Iterator {
//     type Item;
//
//     fn next(&mut self) -> Option<Self::Item>;
