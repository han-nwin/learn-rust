use std::cell::RefCell;
use std::rc::Rc;

// To start, we’ll build a tree with nodes that know about their child nodes.
// We’ll create a struct named Node that holds its own i32 value as well as references to its child Node values:
#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}
