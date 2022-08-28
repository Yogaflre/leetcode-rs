// <三维形体的表面积>

// You are given an n x n grid where you have placed some 1 x 1 x 1 cubes. Each value v = grid[i][j] represents a tower of v cubes placed on top of cell (i, j).
// After placing these cubes, you have decided to glue any directly adjacent cubes to each other, forming several irregular 3D shapes.
// Return the total surface area of the resulting shapes.
// Note: The bottom face of each shape counts toward its surface area.

// Example 1:
// Input: grid = [[1,2],[3,4]]
// Output: 34

// Example 2:
// Input: grid = [[1,1,1],[1,0,1],[1,1,1]]
// Output: 32

// Example 3:
// Input: grid = [[2,2,2],[2,1,2],[2,2,2]]
// Output: 46

// Constraints:
// n == grid.length == grid[i].length
// 1 <= n <= 50
// 0 <= grid[i][j] <= 50

struct Solution;

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut area = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] > 0 {
                    area += grid[i][j] * 4 + 2;
                }
                if i > 0 {
                    area -= (std::cmp::min(grid[i - 1][j], grid[i][j])) * 2;
                }
                if j > 0 {
                    area -= (std::cmp::min(grid[i][j - 1], grid[i][j])) * 2;
                }
            }
        }
        return area;
    }
}
