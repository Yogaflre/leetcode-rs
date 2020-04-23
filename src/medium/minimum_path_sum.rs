// <最短路径和>
// Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right which minimizes the sum of all numbers along its path.
// Note: You can only move either down or right at any point in time.

// Example:
// Input:
// [
//   [1,3,1],
//   [1,5,1],
//   [4,2,1]
// ]
// Output: 7

// Explanation: Because the path 1→3→1→1→1 minimizes the sum.

// 解题思路
// 方法一：动态规划(状态表转移法)
//  构建状态转移表，
// 方法二：动态规划(状态转移方程法)
//  递归并填充状态表
// 方法三：回溯法
//  遍历所欲情况
use std::cmp::min;
struct Solution;
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut matrix = vec![vec![0; grid[0].len()]; grid.len()];
        // 构造状态表(每个位置都是最优解)
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if i == 0 && j == 0 {
                    matrix[i][j] = grid[i][j];
                } else if i == 0 {
                    matrix[i][j] = grid[i][j] + matrix[i][j - 1];
                } else if j == 0 {
                    matrix[i][j] = grid[i][j] + matrix[i - 1][j];
                } else {
                    matrix[i][j] = grid[i][j] + min(matrix[i - 1][j], matrix[i][j - 1]);
                }
            }
        }
        // 获取状态表最后一个元素
        return matrix[grid.len() - 1][grid[0].len() - 1];
    }

    // 状态转移方程法(min_dist(i, j) = w[i][j] + min(min_dist(i, j-1), min_dist(i-1, j)))
    pub fn min_path_sum2(grid: Vec<Vec<i32>>) -> i32 {
        let mut matrix: Vec<Vec<i32>> = vec![vec![-1; grid[0].len()]; grid.len()];
        return Solution::path_sum2(&grid, grid.len() - 1, grid[0].len() - 1, &mut matrix);
    }
    fn path_sum2(grid: &Vec<Vec<i32>>, i: usize, j: usize, matrix: &mut Vec<Vec<i32>>) -> i32 {
        if matrix[i][j] != -1 {
            return matrix[i][j];
        }
        let tmp: i32;
        if i == 0 && j == 0 {
            tmp = grid[i][j];
        } else if i == 0 {
            tmp = grid[i][j] + Solution::path_sum2(grid, i, j - 1, matrix);
        } else if j == 0 {
            tmp = grid[i][j] + Solution::path_sum2(grid, i - 1, j, matrix);
        } else {
            tmp = grid[i][j]
                + min(
                    Solution::path_sum2(grid, i, j - 1, matrix),
                    Solution::path_sum2(grid, i - 1, j, matrix),
                );
        }
        matrix[i][j] = tmp;
        return tmp;
    }

    // 暴力搜索
    pub fn min_path_sum3(grid: Vec<Vec<i32>>) -> i32 {
        return Solution::path_sum3(&grid, grid.len() - 1, grid[0].len() - 1);
    }
    fn path_sum3(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        if i == 0 && j == 0 {
            return grid[i][j];
        } else if i == 0 {
            return grid[i][j] + Solution::path_sum3(grid, i, j - 1);
        } else if j == 0 {
            return grid[i][j] + Solution::path_sum3(grid, i - 1, j);
        } else {
            return grid[i][j]
                + min(
                    Solution::path_sum3(grid, i, j - 1),
                    Solution::path_sum3(grid, i - 1, j),
                );
        }
    }
}

#[test]
fn run() {
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    // assert_eq!(Solution::min_path_sum(grid), 7);
    assert_eq!(Solution::min_path_sum2(grid), 7);
}
