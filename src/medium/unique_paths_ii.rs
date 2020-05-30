// <不同路径2>

// A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
// The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
// Now consider if some obstacles are added to the grids. How many unique paths would there be?

// An obstacle and empty space is marked as 1 and 0 respectively in the grid.
// Note: m and n will be at most 100.

// Example 1:
// Input:
// [
//   [0,0,0],
//   [0,1,0],
//   [0,0,0]
// ]
// Output: 2
// Explanation:
// There is one obstacle in the middle of the 3x3 grid above.
// There are two ways to reach the bottom-right corner:
// 1. Right -> Right -> Down -> Down
// 2. Down -> Down -> Right -> Right

struct Solution;
impl Solution {
    /**
     * 动态规划
     * 向右、向下逐步缩小问题规模
     * 递归终止条件：
     *  1.grid[i][j]==1，道路阻碍 -> 0
     *  2.matrix[i][j]!=0，该路径已经走过 -> 获取结果
     *  3.走到终点 -> 1
     */
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid.is_empty() {
            return 0;
        }
        let mut matrix: Vec<Vec<i32>> = vec![vec![0; obstacle_grid[0].len()]; obstacle_grid.len()];
        return Self::recursive(&obstacle_grid, 0, 0, &mut matrix);
    }
    fn recursive(grid: &Vec<Vec<i32>>, i: usize, j: usize, matrix: &mut Vec<Vec<i32>>) -> i32 {
        if matrix[i][j] != 0 {
            return matrix[i][j];
        }
        if grid[i][j] == 1 {
            return 0;
        }
        if i == grid.len() - 1 && j == grid[0].len() - 1 {
            return 1;
        }
        let mut paths = 0;
        if i + 1 < grid.len() {
            paths += Self::recursive(grid, i + 1, j, matrix);
        }
        if j + 1 < grid[0].len() {
            paths += Self::recursive(grid, i, j + 1, matrix);
        }
        matrix[i][j] = paths;
        return paths;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
        2
    );
}
