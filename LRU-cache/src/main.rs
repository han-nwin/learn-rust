use std::collections::HashMap;

struct Node {
    next: Option<Box<Node>>, //pointer
    prev: Option<Box<Node>>, //pointer
    value: Option<i64>,
}

struct LRUCache {
    capacity: i32,
    map: HashMap<i32, Node>, // key:val = number:Node
    head: Node,
    tail: Node,
}

impl LRUCache {
    fn new(capacity: i32) -> LRUCache {
        let mut map = HashMap::new();
        let mut head = Node {
            next: None,
            prev: None,
            value: None,
        };
        let mut tail = Node {
            next: None,
            prev: None,
            value: None,
        };

        head.next = Some(Box::new(tail));

        tail.prev = Some(Box::new(head));

        LRUCache {
            capacity,
            map,
            head,
            tail,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
