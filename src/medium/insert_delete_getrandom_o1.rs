// <常数时间插入、删除和获取随机元素>

// Design a data structure that supports all following operations in average O(1) time.

// insert(val): Inserts an item val to the set if not already present.
// remove(val): Removes an item val from the set if present.
// getRandom: Returns a random element from current set of elements. Each element must have the same probability of being returned.

// Example:
// Init an empty set.
// RandomizedSet randomSet = new RandomizedSet();
// Inserts 1 to the set. Returns true as 1 was inserted successfully.
// randomSet.insert(1);
// Returns false as 2 does not exist in the set.
// randomSet.remove(2);
// Inserts 2 to the set, returns true. Set now contains [1,2].
// randomSet.insert(2);
// getRandom should return either 1 or 2 randomly.
// randomSet.getRandom();
// Removes 1 from the set, returns true. Set now contains [2].
// randomSet.remove(1);
// 2 was already in the set, so return false.
// randomSet.insert(2);
// Since 2 is the only number in the set, getRandom always return 2.
// randomSet.getRandom();

use rand::{thread_rng, Rng};
use std::collections::HashMap;
/**
 * map存储值和list中的索引位置(hash表快速查找元素)
 * list中存储不同的值(根据索引位置随机获取元素)
 * 不使用hashset是因为不支持根据索引位置访问元素
 */
#[derive(Debug)]
struct RandomizedSet {
    map: HashMap<i32, usize>,
    list: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        RandomizedSet {
            map: HashMap::new(),
            list: vec![0],
        }
    }
    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        } else {
            self.map.insert(val, self.list.len());
            self.list.push(val);
            return true;
        }
    }
    /**
     * Removes a value from the set. Returns true if the set contained the specified element.
     * 将被删除的元素和最后一个元素调换位置，并修改map中的value
     * 删除最后一个元素(因为list删除最后一个元素可以防止元素index移动)
     */
    fn remove(&mut self, val: i32) -> bool {
        match self.map.get_mut(&val) {
            None => return false,
            Some(&mut v) => {
                if v != self.map.len() {
                    self.list.swap(v, self.map.len());
                    self.map.insert(self.list[v], v);
                }
                self.map.remove(&val);
                self.list.pop();
                return true;
            }
        }
    }
    /** Get a random element from the set. */
    fn get_random(&self) -> i32 {
        let rand: usize = thread_rng().gen_range(1, self.list.len());
        return self.list[rand];
    }
}

#[test]
fn run() {
    let mut set = RandomizedSet::new();
    assert_eq!(set.insert(1), true);
    assert_eq!(set.remove(2), false);
    assert_eq!(set.insert(2), true);
    println!("{}", set.get_random());
    assert_eq!(set.remove(1), true);
    assert_eq!(set.insert(2), false);
    println!("{}", set.get_random());
    println!("{}", set.get_random());
}
