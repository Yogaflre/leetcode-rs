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

// 解题思路
// 方法一：HashMap + 双端链表
//  使用HashMap来存储<k,v>元素，使用vec来存储优先级。put和get时间复杂度都为O(n)
// 方法二：
//  TODO
use std::collections::{HashMap, VecDeque};

struct LRUCache {
    capacity: usize,
    items: HashMap<i32, i32>,
    order: VecDeque<i32>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        return LRUCache {
            capacity: capacity as usize,
            items: HashMap::with_capacity(capacity as usize),
            order: VecDeque::new(),
        };
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(value) = self.items.get(&key) {
            if let Some(index) = self.order.iter().position(|&x| x == key) {
                self.order.remove(index);
            }
            self.order.push_front(key);
            return *value;
        }
        return -1;
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.items.contains_key(&key) {
            if let Some(index) = self.order.iter().position(|&x| x == key) {
                self.order.remove(index);
            }
        } else if self.items.len() == self.capacity {
            if let Some(remove) = self.order.pop_back() {
                self.items.remove(&remove);
            }
        }
        self.items.insert(key, value);
        self.order.push_front(key);
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
