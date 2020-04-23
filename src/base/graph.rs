// <图>
use std::collections::LinkedList;

#[derive(PartialEq, Debug)]
struct Graph {
    count: usize,
    adj: Vec<LinkedList<usize>>,
}

impl Graph {
    pub fn create(n: usize) -> Graph {
        let mut adj: Vec<LinkedList<usize>> = vec![];
        for _ in 0..n {
            adj.push(LinkedList::new());
        }
        return Graph {
            count: adj.len(),
            adj: adj,
        };
    }

    pub fn addEdge(&mut self, i: usize, j: usize) {
        self.adj[i].push_back(j);
        self.adj[j].push_back(i);
    }

    // 广度优先(队列)
    // 1.构造items状态表，记录每个节点的最短路径和是否被遍历
    // 2.构造nodes队列，用于记录当前层级(广度)的所有节点
    pub fn bfs(&self, i: usize, j: usize) -> Vec<usize> {
        // 初始化items数组
        let mut items: Vec<Vec<usize>> = vec![vec![]; self.count];
        items[i] = vec![i];
        // 初始化层级队列
        let mut queue: LinkedList<usize> = LinkedList::new();
        queue.push_front(i);
        // 开始从第一个节点遍历
        while !queue.is_empty() {
            if let Some(index) = queue.pop_back() {
                let linked_list: &LinkedList<usize> = &self.adj[index];
                // 和位置index有直接关系的节点入队列
                for node in linked_list {
                    if items[*node].is_empty() {
                        items[*node] = items[index].clone();
                        items[*node].push(*node);
                        // 如果该节点未被遍历，加入到层级队列中
                        queue.push_front(*node);
                    } else {
                        if items[*node].iter().sum::<usize>()
                            > items[index].iter().sum::<usize>() + node
                        {
                            items[*node] = items[index].clone();
                            items[*node].push(*node);
                        }
                    }
                }
            }
        }
        return items[j].to_owned();
    }

    // 深度优先(回溯法)，一条分支走到底，先递归，持久化重复子问题
    pub fn dfs(&self, i: usize, j: usize) -> Vec<usize> {
        // 初始化items数组
        let mut items: Vec<Vec<usize>> = vec![vec![]; self.count];
        items[i] = vec![i];
        self.dfs_recursive(i, j, &mut items);
        return items[j].to_owned();
    }
    fn dfs_recursive(&self, i: usize, j: usize, items: &mut Vec<Vec<usize>>) {
        let linked_list: &LinkedList<usize> = &self.adj[i];
        for n in linked_list.iter() {
            if items[*n].is_empty() {
                items[*n] = items[i].clone();
                items[*n].push(*n);
                self.dfs_recursive(*n, j, items);
            } else {
                if items[*n].iter().sum::<usize>() > items[i].iter().sum::<usize>() + n {
                    items[*n] = items[i].clone();
                    items[*n].push(*n);
                }
            }
        }
    }
}

#[test]
fn run() {
    let mut graph = Graph::create(8);
    graph.addEdge(0, 1);
    graph.addEdge(0, 3);
    graph.addEdge(1, 2);
    graph.addEdge(1, 4);
    graph.addEdge(2, 5);
    graph.addEdge(3, 4);
    graph.addEdge(4, 5);
    graph.addEdge(4, 6);
    graph.addEdge(5, 7);
    graph.addEdge(6, 7);
    println!("{:?}", graph);

    // 找到i->j节点的最短路径
    let short_path = vec![0, 1, 2, 5, 7];

    let bfs_short_path = graph.bfs(0, 7);
    assert_eq!(bfs_short_path, short_path);
    let dfs_short_path = graph.dfs(0, 7);
    assert_eq!(dfs_short_path, short_path);
}
