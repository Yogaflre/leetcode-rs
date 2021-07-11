// <LFU缓存>

// Design and implement a data structure for Least Frequently Used (LFU) cache. It should support the following operations: get and put.

// get(key) - Get the value (will always be positive) of the key if the key exists in the cache, otherwise return -1.
// put(key, value) - Set or insert the value if the key is not already present. When the cache reaches its capacity, it should invalidate the least frequently used item before inserting a new item. For the purpose of this problem, when there is a tie (i.e., two or more keys that have the same frequency), the least recently used key would be evicted.

// Note that the number of times an item is used is the number of calls to the get and put functions for that item since it was inserted. This number is set to zero when the item is removed.
// Follow up:
// Could you do both operations in O(1) time complexity?

// Example:
// LFUCache cache = new LFUCache( 2 /* capacity */ );
// cache.put(1, 1);
// cache.put(2, 2);
// cache.get(1);       // returns 1
// cache.put(3, 3);    // evicts key 2
// cache.get(2);       // returns -1 (not found)
// cache.get(3);       // returns 3.
// cache.put(4, 4);    // evicts key 1.
// cache.get(1);       // returns -1 (not found)
// cache.get(3);       // returns 3
// cache.get(4);       // returns 4

use std::collections::HashMap;
/**
 * LFU特性：最近不经常使用淘汰策略
 * 构造LFUCache结构：capacity容量、map存储kv结构，并且存储当前k被访问的次数，和最新修改的标签（可以是时间戳）
 */
#[derive(Debug)]
struct LFUCache {
    capacity: usize,
    map: HashMap<i32, (i32, usize, usize)>,
    //NOTE 使map value中的第三个元素可以用时间戳代替，rust获取时间戳比较麻烦，并且在leetcode会出现错乱的现象。所以可以用全局自增ID代替
    flag: usize,
}
impl LFUCache {
    fn new(capacity: i32) -> Self {
        LFUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            flag: 0,
        }
    }

    /**
     * 如果map中存在，则更新计数器和时间戳
     */
    fn get(&mut self, key: i32) -> i32 {
        self.flag += 1;
        if let Some((v, n, t)) = self.map.get_mut(&key) {
            *n += 1;
            *t = self.flag;
            return *v;
        } else {
            return -1;
        }
    }

    /**
     * 当容量为0时，直接忽略
     * 当容量已满：并且k不存在时，先删除使用次数最少的entry(若次数相同，则删除时间戳最小的)
     * 插入新元素：如果k存在，则更新计数器和时间戳。否则，插入新元组
     */
    fn put(&mut self, key: i32, value: i32) {
        if self.capacity != 0 {
            return;
        }
        if self.map.len() == self.capacity && !self.map.contains_key(&key) {
            let val: usize = self.map.values().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
            let key: i32 = *self
                .map
                .iter()
                .filter(|x| (x.1).1 == val)
                .min_by(|a, b| (a.1).2.cmp(&(b.1).2))
                .unwrap()
                .0;
            self.map.remove(&key);
        }
        self.flag += 1;
        if let Some((v, n, t)) = self.map.get_mut(&key) {
            *v = value;
            *n += 1;
            *t = self.flag;
        } else {
            self.map.insert(key, (value, 0, self.flag));
        }
    }
}

#[test]
fn run() {
    let mut cache = LFUCache::new(2);
    // cache.put(1, 1);
    // cache.put(2, 2);
    // assert_eq!(cache.get(1), 1);
    // cache.put(3, 3);
    // assert_eq!(cache.get(2), -1);
    // assert_eq!(cache.get(3), 3);
    // cache.put(4, 4);
    // assert_eq!(cache.get(1), -1);
    // assert_eq!(cache.get(3), 3);
    // assert_eq!(cache.get(4), 4);

    assert_eq!(cache.get(2), -1);
    cache.put(2, 6);
    assert_eq!(cache.get(1), -1);
    cache.put(1, 5);
    cache.put(1, 2);
    assert_eq!(cache.get(1), 2);
    assert_eq!(cache.get(2), 6);
}
