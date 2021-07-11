// <二进制矩阵中的最短路径>

// Given an n x n binary matrix grid, return the length of the shortest clear path in the matrix. If there is no clear path, return -1.
// A clear path in a binary matrix is a path from the top-left cell (i.e., (0, 0)) to the bottom-right cell (i.e., (n - 1, n - 1)) such that:
// All the visited cells of the path are 0.
// All the adjacent cells of the path are 8-directionally connected (i.e., they are different and they share an edge or a corner).
// The length of a clear path is the number of visited cells of this path.

// Example 1:
// Input: grid = [[0,1],
//                [1,0]]
// Output: 2

// Example 2:
// Input: grid = [[0,0,0],
//                [1,1,0],
//                [1,1,0]]
// Output: 4

// Example 3:
// Input: grid = [[1,0,0],
//                [1,1,0],
//                [1,1,0]]
// Output: -1

// Constraints:
// n == grid.length
// n == grid[i].length
// 1 <= n <= 100
// grid[i][j] is 0 or 1

use std::collections::VecDeque;
struct Solution;
impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return -1;
        }
        let mut grid = grid;
        return Self::bfs(&mut grid);
    }

    /*
     * 广度优先遍历
     */
    fn bfs(grid: &mut Vec<Vec<i32>>) -> i32 {
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        let tmp = vec![-1, 0, 1];
        let mut res = 0;
        queue.push_back((0, 0));
        while !queue.is_empty() {
            // NOTE queue不为空说明当前节点合法，需要记录当前路径到结果中
            res += 1;
            for _ in 0..queue.len() {
                let pair: (i32, i32) = queue.pop_front().unwrap();
                if grid[pair.0 as usize][pair.1 as usize] != 1 {
                    // NOTE 如果遍历到最后一个节点则直接返回结果
                    if pair.0 + 1 == grid.len() as i32 && pair.1 + 1 == grid[0].len() as i32 {
                        return res;
                    }
                    // NOTE 递归之前需要把已经走过的路径变为1，防止重复计算
                    grid[pair.0 as usize][pair.1 as usize] = 1;
                    for l in tmp.iter() {
                        for r in tmp.iter() {
                            let i = pair.0 + l;
                            let j = pair.1 + r;
                            if i >= 0
                                && i < grid.len() as i32
                                && j >= 0
                                && j < grid[i as usize].len() as i32
                            {
                                queue.push_back((i, j));
                            }
                        }
                    }
                }
            }
        }
        return -1;
    }

    /*
     * FIXME 为什么dfs + dp不行？
     */
    fn dp(grid: &mut Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
        // 不符合条件直接返回-1
        if i < 0
            || i >= grid.len() as i32
            || j < 0
            || j >= grid[i as usize].len() as i32
            || grid[i as usize][j as usize] == 1
        {
            return -1;
        }
        let i = i as usize;
        let j = j as usize;
        // 遍历到最后一个节点时结束遍历，并返回1
        if i + 1 == grid.len() && j + 1 == grid[i].len() {
            return 1;
        }
        // 如果dp中有值，说明是当前节点最小值。直接返回
        if dp[i][j] != 0 {
            return dp[i][j];
        }
        // NOTE 在单次寻找路径中，要记录已经走过的路径，防止倒退
        grid[i][j] = 1;
        // 寻找8个方向路径上的最小值
        let mut count = i32::max_value();
        let direction: Vec<i32> = vec![-1, 0, 1];
        for l in direction.iter() {
            for r in direction.iter() {
                let tmp = Self::dp(grid, dp, i as i32 + l, j as i32 + r);
                if tmp != -1 && tmp < count {
                    count = tmp;
                }
            }
        }
        // NOTE 单次寻找路径结束时，应该将走过的路径标志删除。以便计算新的路径
        grid[i][j] = 0;
        if count != i32::max_value() {
            dp[i][j] = count + 1;
        } else {
            dp[i][j] = -1;
        }
        return dp[i][j];
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::shortest_path_binary_matrix(vec![vec![0, 1], vec![1, 0]]),
        2
    );
    assert_eq!(
        Solution::shortest_path_binary_matrix(vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]]),
        4
    );
    assert_eq!(
        Solution::shortest_path_binary_matrix(vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 1]]),
        -1
    );
    assert_eq!(
        Solution::shortest_path_binary_matrix(vec![
            vec![0, 1, 1, 1],
            vec![1, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 0, 0]
        ]),
        4
    );
    assert_eq!(
        Solution::shortest_path_binary_matrix(vec![
            vec![0, 0, 1, 0, 1, 1],
            vec![1, 0, 0, 1, 0, 0],
            vec![0, 1, 0, 1, 0, 0],
            vec![1, 0, 1, 0, 0, 0],
            vec![0, 1, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ]),
        6
    );
}
