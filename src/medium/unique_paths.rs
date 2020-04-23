// <不同路径>
// A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
// The robot can only move either down or right at any point in time.
// The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
// How many possible unique paths are there?

// Example 1:
// Input: m = 3, n = 2
// Output: 3

// Explanation:
// From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
// 1. Right -> Right -> Down
// 2. Right -> Down -> Right
// 3. Down -> Right -> Right

// Example 2:
// Input: m = 7, n = 3
// Output: 28

// 解题思路
// 方法一：动态规划
//  初始化一个m*n并且值全为1的矩阵，从(1,1)的位置开始遍历，将该元素的左侧+上侧赋值为该节点。遍历完成后返回最后一个节点的值
// 方法二：递归（耗时）
//  当 m或者n等于1时，代表无法再进行缩减，只有唯一路径，所以返回1。否则m和n分别减一进行递归
struct Solution;
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut matrix = vec![vec![1; m as usize]; n as usize];
        for i in 1..n as usize {
            for j in 1..m as usize {
                matrix[i][j] = matrix[i - 1][j] + matrix[i][j - 1];
            }
        }
        return matrix[n as usize - 1][m as usize - 1];
    }

    // WHY 过不了leetcode（为什么这么耗时呢？）
    pub fn unique_paths_recursive(m: i32, n: i32) -> i32 {
        if m == 1 || n == 1 {
            return 1;
        } else {
            return Solution::unique_paths_recursive(m, n - 1)
                + Solution::unique_paths_recursive(m - 1, n);
        }
    }
}

#[test]
fn run() {
    assert_eq!(Solution::unique_paths(3, 2), 2);
    assert_eq!(Solution::unique_paths(7, 3), 28);
    assert_eq!(Solution::unique_paths(51, 9), 1916797311);
}
