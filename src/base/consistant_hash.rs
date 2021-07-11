// <一致性hash算法>

use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct ConsistantHash {
    // 虚拟节点Map(需要用到有序的Map，适合转移节点)
    virtual_nodes: BTreeMap<u64, Node>,
    // 物理节点
    nodes: Vec<String>,
    // 虚拟节点复制个数
    replica: u8,
}
#[derive(Debug)]
struct Node {
    // 物理节点id
    address: String,
    // 具体值(模拟虚拟结点存放的具体值，实际应该存放在一个物理节点中)
    values: HashMap<String, String>,
}

impl ConsistantHash {
    /**
     * 创建hash环
     */
    fn create(nodes: Vec<String>, replica: u8) -> ConsistantHash {
        let mut virtual_nodes: BTreeMap<u64, Node> = BTreeMap::new();
        for address in &nodes {
            for i in 0..replica {
                let mut key = address.clone();
                key.push_str(&i.to_string());
                virtual_nodes.insert(
                    Self::hash_virtual(&key),
                    Node {
                        address: address.clone(),
                        values: HashMap::new(),
                    },
                );
            }
        }
        return ConsistantHash {
            nodes,
            virtual_nodes,
            replica,
        };
    }

    /**
     * 计算索引值(默认一共128个节点)
     */
    fn hash_virtual(key: &String) -> u64 {
        let mut hasher = DefaultHasher::default();
        key.hash(&mut hasher);
        return hasher.finish() % (1 << 7);
    }

    /**
     * 根据key查找第一个虚拟节点
     */
    fn find_first_node(&mut self, key: &String) -> &mut Node {
        let index = Self::hash_virtual(&key);
        return self.find_first_node_by_index(index);
    }

    /**
     * 根据index查找第一个虚拟节点
     */
    fn find_first_node_by_index(&mut self, index: u64) -> &mut Node {
        if self.virtual_nodes.range_mut(index..).next().is_some() {
            return self.virtual_nodes.range_mut(index..).next().unwrap().1;
        } else {
            return self
                .virtual_nodes
                .range_mut(0..)
                .next()
                .expect("virtual_nodes is empty!")
                .1;
        }
    }

    /**
     * 添加值
     */
    fn add(&mut self, k: String, v: String) {
        let node: &mut Node = self.find_first_node(&k);
        node.values.insert(k, v);
    }

    /**
     * 删除元素
     */
    fn delete(&mut self, k: String) {
        let node: &mut Node = self.find_first_node(&k);
        node.values.remove(&k);
    }

    /**
     * 添加结点
     */
    fn add_node(&mut self, address: String) {
        for i in 0..self.replica {
            let mut key = address.clone();
            key.push_str(&i.to_string());
            self.virtual_nodes.insert(
                Self::hash_virtual(&key),
                Node {
                    address: address.clone(),
                    values: HashMap::new(),
                },
            );
        }
    }

    /**
     * 删除节点
     * 将该节点的值全部移动到下一个虚拟结点
     */
    fn delete_node(&mut self, address: String) {
        for i in 0..self.replica {
            let mut target = address.clone();
            target.push_str(&i.to_string());
            let index = &Self::hash_virtual(&target);
            if let Some(node) = self.virtual_nodes.remove(&index) {
                let next: &mut Node = self.find_first_node_by_index(index + 1);
                for (k, v) in node.values {
                    next.values.insert(k, v);
                }
            }
        }
    }
}

#[test]
fn run() {
    let mut nodes = ConsistantHash::create(
        vec![
            "localhost:8000".to_string(),
            "localhost:8001".to_string(),
            "localhost:8002".to_string(),
        ],
        2,
    );
    nodes.add("0".to_string(), "0".to_string());
    nodes.add("1".to_string(), "1".to_string());
    nodes.add("hi".to_string(), "ok".to_string());
    nodes.add("127".to_string(), "127".to_string());
    println!("--------------------------\n{:#?}", nodes);

    nodes.delete("0".to_string());
    println!("--------------------------\n{:#?}", nodes);

    nodes.add_node("localhost:8003".to_string());
    println!("--------------------------\n{:#?}", nodes);

    nodes.delete_node("localhost:8002".to_string());
    println!("--------------------------\n{:#?}", nodes);
}
