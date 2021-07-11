// 构建Java版 HashMap

use std::cmp::min;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct JavaMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    // 当前kv个数
    size: u64,
    // 装载因子
    load_factor: f32,
    // 阈值 = 装载因子 * 个数
    threshold: u64,
    // 初始化容量
    capacity: u64,
    // 节点(数组+链表)，使用vec模拟链表，rust中的LinkedList没有删除操作
    nodes: Vec<Vec<(K, V)>>,
}

impl<K, V> JavaMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn default() -> JavaMap<K, V> {
        return JavaMap {
            size: 0,
            load_factor: 0.75,
            threshold: Self::get_threshold(16, 0.75),
            capacity: 16,
            nodes: vec![vec![]; 16],
        };
    }
    fn new(capacity: u64) -> JavaMap<K, V> {
        let capacity = Self::init_capacity(capacity);
        return JavaMap {
            size: 0,
            load_factor: 0.75,
            threshold: Self::get_threshold(capacity, 0.75),
            capacity,
            nodes: vec![vec![]; capacity as usize],
        };
    }

    fn size(&self) -> u64 {
        return self.size;
    }

    /**
     * 初始化容量为2的指数
     */
    fn init_capacity(capacity: u64) -> u64 {
        let mut n = capacity - 1;
        n = n | n >> 1;
        n = n | n >> 2;
        n = n | n >> 4;
        n = n | n >> 8;
        n = n | n >> 16;
        return min(u64::max_value(), n + 1);
    }

    fn get_threshold(capacity: u64, load_factor: f32) -> u64 {
        return (capacity as f32 * load_factor) as u64;
    }

    /**
     * 获取hash值
     */
    fn get_index(k: &K, capacity: u64) -> usize {
        let mut hasher = DefaultHasher::default();
        k.hash(&mut hasher);
        //NOTE 使用与运算优化取模运算，capacity必须是2的指数
        let index = hasher.finish() & (capacity - 1);
        return index as usize;
    }

    fn get(&mut self, k: K) -> Option<&V> {
        let index = Self::get_index(&k, self.capacity);
        if let Some(list) = self.nodes.get(index) {
            for (ko, v) in list.iter() {
                if k.eq(ko) {
                    return Some(v);
                }
            }
        }
        return None;
    }

    fn put(&mut self, k: K, v: V) {
        let index = Self::get_index(&k, self.capacity);
        // 插入nodes中
        if let Some(list) = self.nodes.get_mut(index) {
            loop {
                if let Some((ko, vo)) = list.iter_mut().next() {
                    if k.eq(ko) {
                        *vo = v;
                        break;
                    }
                } else {
                    list.push((k, v));
                    break;
                }
            }
        }
        self.size += 1;
        // 判断是否需要扩容
        if self.size > self.threshold {
            self.resize();
        }
    }

    /**
     * 扩容
     */
    fn resize(&mut self) {
        self.capacity = self.capacity << 1;
        self.threshold = Self::get_threshold(self.capacity, self.load_factor);
        let mut nodes: Vec<Vec<(K, V)>> = vec![vec![]; self.capacity as usize];
        for list in self.nodes.iter_mut() {
            while let Some((ko, vo)) = list.pop() {
                let index = Self::get_index(&ko, self.capacity);
                nodes[index].push((ko, vo));
            }
        }
        self.nodes = nodes;
    }

    fn remove(&mut self, k: K) {
        let index = Self::get_index(&k, self.capacity);
        if let Some(list) = self.nodes.get_mut(index) {
            for (i, (ko, _)) in list.iter().enumerate() {
                if k.eq(ko) {
                    list.remove(i);
                    self.size -= 1;
                    return;
                }
            }
        }
    }
}

#[test]
fn run() {
    // 初始化容量为1
    let mut map: JavaMap<i32, i32> = JavaMap::new(1);
    println!("map:{:#?}", map);
    assert_eq!(map.capacity, 1);

    // 扩容
    map.put(1, 10);
    println!("map:{:#?}", map);
    assert_eq!(map.capacity, 2);
    assert_eq!(*map.get(1).unwrap(), 10);

    // 扩容
    map.put(2, 20);
    println!("map:{:#?}", map);
    assert_eq!(map.capacity, 4);
    assert_eq!(*map.get(2).unwrap(), 20);

    // 不会扩容
    map.put(5, 50);
    println!("map:{:#?}", map);
    assert_eq!(map.capacity, 4);
    assert_eq!(*map.get(5).unwrap(), 50);

    // 删除
    map.remove(5);
    assert_eq!(map.get(5), None);
    println!("map:{:#?}", map);
}
