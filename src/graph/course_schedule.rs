// <课程表>

// There are a total of numCourses courses you have to take, labeled from 0 to numCourses-1.
// Some courses may have prerequisites, for example to take course 0 you have to first take course 1, which is expressed as a pair: [0,1]
// Given the total number of courses and a list of prerequisite pairs, is it possible for you to finish all courses?

// Example 1:
// Input: numCourses = 2, prerequisites = [[1,0]]
// Output: true
// Explanation: There are a total of 2 courses to take.
//              To take course 1 you should have finished course 0. So it is possible.

// Example 2:
// Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
// Output: false
// Explanation: There are a total of 2 courses to take.
//              To take course 1 you should have finished course 0, and to take course 0 you should
//              also have finished course 1. So it is impossible.

// Constraints:
// The input prerequisites is a graph represented by a list of edges, not adjacency matrices. Read more about how a graph is represented.
// You may assume that there are no duplicate edges in the input prerequisites.
// 1 <= numCourses <= 10^5

use std::collections::LinkedList;
struct Solution;
impl Solution {
    /*
     * 图(dfs)
     * 构造邻接表 -> 遍历图 -> 校验图中是否有环(初始化flags数组)
     * flatgs：1为当前遍历头节点；0为未遍历节点；-1为已遍历节点
     */
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // 构造图
        let mut graph: Vec<LinkedList<i32>> = vec![LinkedList::new(); num_courses as usize];
        for v in prerequisites {
            let pre = v[0] as usize;
            let last = v[1];
            graph[pre].push_front(last);
        }
        // 遍历图
        let mut flags: Vec<i32> = vec![0; num_courses as usize];
        for i in 0..graph.len() {
            if !Self::dfs(&graph, &mut flags, i) {
                return false;
            }
        }
        return true;
    }

    fn dfs(graph: &Vec<LinkedList<i32>>, flags: &mut Vec<i32>, i: usize) -> bool {
        if flags[i] == -1 {
            return true;
        }
        if flags[i] == 1 {
            return false;
        }
        flags[i] = 1;
        let list = &graph[i];
        for l in list {
            if !Self::dfs(graph, flags, *l as usize) {
                return false;
            }
        }
        flags[i] = -1;
        return true;
    }
}
