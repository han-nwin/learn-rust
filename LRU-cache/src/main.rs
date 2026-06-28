use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    next: Option<Rc<RefCell<Node>>>, //pointer or None
    prev: Option<Rc<RefCell<Node>>>, //pointer or None
    value: Option<usize>,
    key: Option<i32>,
}

struct LRUCache {
    capacity: i32,
    map: HashMap<i32, Rc<RefCell<Node>>>, // key:val = number:Node_ptr
    head: Rc<RefCell<Node>>,              // pointer to data on heap
    tail: Rc<RefCell<Node>>,              // pointer to data on heap
}

impl LRUCache {
    fn new(capacity: i32) -> LRUCache {
        // not allow capacity <= 0
        eprintln!("Capacity has to be greater than 0");
        assert!(capacity > 0);

        let map = HashMap::new();
        let head = Rc::new(RefCell::new(Node {
            next: None,
            prev: None,
            value: None,
            key: None,
        }));
        let tail = Rc::new(RefCell::new(Node {
            next: None,
            prev: None,
            value: None,
            key: None,
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
        let node = self.map.get(&key)?; // Option<> here
        // Move it to the front
        // break the link
        let before = node.borrow().prev.clone();
        let after = node.borrow().next.clone();

        if let Some(before_rc) = &before {
            before_rc.borrow_mut().next = after.clone();
        }
        if let Some(after_rc) = &after {
            after_rc.borrow_mut().prev = before.clone();
        }
        //connect to head
        let node_after_head = self.head.borrow().next.clone();
        node.borrow_mut().next = node_after_head.clone();
        if let Some(node_after_head_rc) = node_after_head {
            node_after_head_rc.borrow_mut().prev = Some(Rc::clone(node))
        }

        self.head.borrow_mut().next = Some(Rc::clone(node));
        node.borrow_mut().prev = Some(Rc::clone(&self.head));

        node.borrow().value // return the copied value. value is Option<usize> so it's Copy

        // if Value is a String we can do
        // let k = node.borrow();
        // let val = Ref::map(k, |node| node.value.as_ref().unwrap());
    }

    // put new node
    fn put(&mut self, key: i32, value: usize) {
        if (self.map.len() as i32) >= self.capacity {
            // === Delete LRU node ===

            // Since capacity > 0 so tail won't be pointing to head here
            // clone() create Option<ref> copy, not data, so it's ok
            let lru_node = self.tail.borrow().prev.clone();

            // if this node exist
            if let Some(lru_node_rc) = lru_node {
                // point the tail to the node before the lru node
                self.tail.borrow_mut().prev = lru_node_rc.borrow().prev.clone();

                // point the node before the lru node to the tail
                let node_before_lru = lru_node_rc.borrow().prev.clone();
                if let Some(node_before_lru_rc) = node_before_lru {
                    node_before_lru_rc.borrow_mut().next = Some(Rc::clone(&self.tail));
                }

                // Delete it here
                lru_node_rc.borrow_mut().next = None;
                lru_node_rc.borrow_mut().prev = None;
                // Delete from the map
                if let Some(lru_node_key) = lru_node_rc.borrow().key {
                    self.map.remove(&lru_node_key);
                }
                drop(lru_node_rc); // drop the data
            }
        }

        //=== Insert to the linked list ===
        // create new Node
        let new_node = Rc::new(RefCell::new(Node {
            next: None,
            prev: None,
            value: Some(value),
            key: Some(key),
        }));
        // get the current node in the front
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

fn main() {
    let mut cache = LRUCache::new(3);

    cache.put(1, 10);
    cache.put(2, 20);
    cache.put(3, 30);

    println!("get 1 = {:?}", cache.get(1)); // Some(10)
    println!("get 2 = {:?}", cache.get(2)); // Some(20)
    println!("get 9 = {:?}", cache.get(9)); // None

    // Cache is full. Since 3 is least recently used now, this should evict key 3.
    cache.put(4, 40);

    println!("after putting 4:");
    println!("get 1 = {:?}", cache.get(1)); // Some(10)
    println!("get 2 = {:?}", cache.get(2)); // Some(20)
    println!("get 3 = {:?}", cache.get(3)); // None if eviction works
    println!("get 4 = {:?}", cache.get(4)); // Some(40)

    println!("map contents:");
    for (key, node_rc) in &cache.map {
        println!("key: {:?}, value: {:?}", key, node_rc.borrow().value);
    }
}
