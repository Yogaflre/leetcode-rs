// <找到最终的安全状态>

// There is a directed graph of n nodes with each node labeled from 0 to n - 1. The graph is represented by a 0-indexed 2D integer array graph where graph[i] is an integer array of nodes adjacent to node i, meaning there is an edge from node i to each node in graph[i].
// A node is a terminal node if there are no outgoing edges. A node is a safe node if every possible path starting from that node leads to a terminal node.
// Return an array containing all the safe nodes of the graph. The answer should be sorted in ascending order.

// Example 1:
// Illustration of graph
// Input: graph = [[1,2],[2,3],[5],[0],[5],[],[]]
// Output: [2,4,5,6]
// Explanation: The given graph is shown above.
// Nodes 5 and 6 are terminal nodes as there are no outgoing edges from either of them.
// Every path starting at nodes 2, 4, 5, and 6 all lead to either node 5 or 6.

// Example 2:
// Input: graph = [[1,2,3,4],[1,2],[3,4],[0,4],[]]
// Output: [4]
// Explanation:
// Only node 4 is a terminal node, and every path starting at node 4 leads to node 4.

// Constraints:
//     n == graph.length
//     1 <= n <= 104
//     0 <= graph[i].length <= n
//     0 <= graph[i][j] <= n - 1
//     graph[i] is sorted in a strictly increasing order.
//     The graph may contain self-loops.
//     The number of edges in the graph will be in the range [1, 4 * 104].

struct Solution;
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut flags = vec![0; graph.len()];
        let mut res = vec![];
        for i in 0..graph.len() {
            Self::dfs(&graph, &mut flags, &mut res, i);
        }
        res.sort();
        return res;
    }

    /*
     * flag: 0-未遍历节点 1-已遍历的节点 2-普通节点 3-终端节点/安全节点
     */
    fn dfs(graph: &Vec<Vec<i32>>, flags: &mut Vec<i32>, res: &mut Vec<i32>, index: usize) -> i32 {
        // 如果节点已经被标识，直接返回
        if flags[index] != 0 {
            return flags[index];
        }
        flags[index] = 1; // 当前节点标记为已遍历节点
        let mut flag = 3; // 假设当前节点为终端节点/安全节点
        for i in &graph[index] {
            // 如果其中一个子节点不是终端节点 -> 将当前节点设置为普通节点
            if Self::dfs(graph, flags, res, *i as usize) != 3 {
                flag = 2;
            }
        }
        if flag != 2 {
            res.push(index as i32);
        }
        flags[index] = flag;
        return flag;
    }
}
