use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    next: Option<Rc<RefCell<Node>>>, //pointer or None
    prev: Option<Rc<RefCell<Node>>>, //pointer or None
    value: Option<usize>,
}

struct LRUCache {
    capacity: i32,
    map: HashMap<i32, Rc<RefCell<Node>>>, // key:val = number:Node_ptr
    head: Rc<RefCell<Node>>,              // pointer to data on heap
    tail: Rc<RefCell<Node>>,              // pointer to data on heap
}

impl LRUCache {
    fn new(capacity: i32) -> LRUCache {
        let map = HashMap::new();
        let head = Rc::new(RefCell::new(Node {
            next: None,
            prev: None,
            value: None,
        }));
        let tail = Rc::new(RefCell::new(Node {
            next: None,
            prev: None,
            value: None,
        }));

        head.borrow_mut().next = Some(Rc::clone(&tail));

        tail.borrow_mut().prev = Some(Rc::clone(&head));

        LRUCache {
            capacity,
            map,
            head: head.clone(), // dummy node
            tail: tail.clone(), // dummy node
        }
    }

    // get a node value
    fn get(&self, key: i32) -> Option<usize> {
        let map_value = self.map.get(&key)?;

        map_value.borrow().value // return a copied value

        // match map_value {
        //     Some(node_rc) => {
        //         let node_ref = node_rc.borrow();
        //         node_ref.value // return a copied value
        //     }
        //     None => None,
        // }
    }

    // put new node
    fn put(&mut self, key: i32, value: usize) {
        if (self.map.len() as i32) < self.capacity {
            // create new Node
            let new_node = Rc::new(RefCell::new(Node {
                next: None,
                prev: None,
                value: Some(value),
            }));

            //=== Insert to the linked list ===
            let old_node = match &self.head.borrow().next {
                Some(node_rc) => Some(Rc::clone(node_rc)),
                None => None,
            };

            self.head.borrow_mut().next = Some(Rc::clone(&new_node));
            new_node.borrow_mut().prev = Some(Rc::clone(&self.head));

            // if head->next exist
            if let Some(old_node_rc) = old_node {
                old_node_rc.borrow_mut().prev = Some(Rc::clone(&new_node));
                new_node.borrow_mut().next = Some(Rc::clone(&old_node_rc));
            }
            //==== End insert to linked list ===

            // Insert to map
            self.map.insert(key, new_node);
        }
    }
}

fn main() {
    let mut new_cache = LRUCache::new(3);
    new_cache.put(1, 10);

    for (key, node_rc) in &new_cache.map {
        println!("key: {:?}, value: {:?}", key, node_rc.borrow().value);
    }
}
