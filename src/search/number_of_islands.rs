// <岛屿数量>

// Given a 2d grid map of '1's (land) and '0's (water), count the number of islands.
// An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically.
// You may assume all four edges of the grid are all surrounded by water.

// Example 1:
// Input:
// 11110
// 11010
// 11000
// 00000
// Output: 1

// Example 2:
// Input:
// 11000
// 11000
// 00100
// 00011
// Output: 3

struct Solution;
impl Solution {
    /**
     * 遍历+覆盖路径
     * 从头到尾遍历二维数组。当节点为1时，count+1，递归所有相邻的位置，并修改遍历过的节点值
     */
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let mut count = 0;
        if grid.is_empty() {
            return count;
        }
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    count += 1;
                    Self::update_islands(&mut grid, i, j);
                }
            }
        }
        return count;
    }

    fn update_islands(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if grid[i][j] == '1' {
            grid[i][j] = '0';
            if i > 0 {
                Self::update_islands(grid, i - 1, j);
            }
            if i < grid.len() - 1 {
                Self::update_islands(grid, i + 1, j);
            }
            if j > 0 {
                Self::update_islands(grid, i, j - 1);
            }
            if j < grid[0].len() - 1 {
                Self::update_islands(grid, i, j + 1);
            }
        }
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::num_islands(vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0']
        ]),
        1
    );
}
