// <LRU缓存(Least Recently Used)>

// Design and implement a data structure for Least Recently Used (LRU) cache. It should support the following operations: get and put.
// get(key) - Get the value (will always be positive) of the key if the key exists in the cache, otherwise return -1.
// put(key, value) - Set or insert the value if the key is not already present.
// When the cache reached its capacity, it should invalidate the least recently used item before inserting a new item.

// The cache is initialized with a positive capacity.

// Follow up:
// Could you do both operations in O(1) time complexity?

// Example:
// LRUCache cache = new LRUCache( 2 /* capacity */ );
// cache.put(1, 1);
// cache.put(2, 2);
// cache.get(1);       // returns 1
// cache.put(3, 3);    // evicts key 2
// cache.get(2);       // returns -1 (not found)
// cache.put(4, 4);    // evicts key 1
// cache.get(1);       // returns -1 (not found)
// cache.get(3);       // returns 3
// cache.get(4);       // returns 4

use std::{cell::RefCell, collections::HashMap, rc::Rc};

/*
 * 自定义双向链表
 */
#[derive(Default)]
struct Node {
    key: i32,
    val: i32,
    pre: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, val: i32) -> Self {
        Node {
            key,
            val,
            pre: None,
            next: None,
        }
    }
}

/*
 * HashMap存储自定义双向链表的节点，用于快速查找
 */
struct LRUCache {
    capacity: usize,
    items: HashMap<i32, Rc<RefCell<Node>>>,
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        return LRUCache {
            capacity: capacity as usize,
            items: HashMap::with_capacity(capacity as usize),
            head: None,
            tail: None,
        };
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.items.get(&key).map(|n| n.clone()) {
            self.removeNode(node.clone());
            self.addNode(node.clone());
            return node.borrow().val;
        }
        return -1;
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(old) = self.items.get(&key).map(|n| n.clone()) {
            self.removeNode(old);
        } else if self.capacity == self.items.len() && self.head.is_some() {
            self.removeNode(self.head.as_ref().unwrap().clone());
        }
        self.addNode(Rc::new(RefCell::new(Node::new(key, value))));
    }

    fn removeNode(&mut self, node: Rc<RefCell<Node>>) {
        self.items.remove(&node.borrow().key);
        let pre: Option<Rc<RefCell<Node>>> = node.borrow().pre.as_ref().map(|n| n.clone());
        let next: Option<Rc<RefCell<Node>>> = node.borrow().next.as_ref().map(|n| n.clone());
        match (pre, next) {
            (Some(p), Some(n)) => {
                p.borrow_mut().next = Some(n.clone());
                n.borrow_mut().pre = Some(p.clone());
            }
            (Some(p), None) => {
                p.borrow_mut().next = None;
                self.tail = Some(p.clone());
            }
            (None, Some(n)) => {
                n.borrow_mut().pre = None;
                self.head = Some(n.clone());
            }
            (None, None) => {
                self.head = None;
                self.tail = None;
            }
        }
    }

    fn addNode(&mut self, node: Rc<RefCell<Node>>) {
        self.items.insert(node.borrow().key, node.clone());
        if self.tail.is_some() {
            node.borrow_mut().pre = self.tail.as_ref().map(|n| n.clone());
            // NOTE 注意一定要把最新添加的node.next设置为None
            node.borrow_mut().next = None;
            self.tail.as_ref().unwrap().borrow_mut().next = Some(node.clone());
        } else {
            self.head = Some(node.clone());
        }
        self.tail = Some(node.clone());
    }
}

#[test]
fn run() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);
    cache.put(3, 3);
    assert_eq!(cache.get(2), -1);
    cache.put(4, 4);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), 4);
}
