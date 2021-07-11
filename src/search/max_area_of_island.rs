// <岛屿的最大面积>

// Given a non-empty 2D array grid of 0's and 1's, an island is a group of 1's (representing land) connected 4-directionally (horizontal or vertical.)
// You may assume all four edges of the grid are surrounded by water.
// Find the maximum area of an island in the given 2D array. (If there is no island, the maximum area is 0.)

// Example 1:
// [[0,0,1,0,0,0,0,1,0,0,0,0,0],
//  [0,0,0,0,0,0,0,1,1,1,0,0,0],
//  [0,1,1,0,1,0,0,0,0,0,0,0,0],
//  [0,1,0,0,1,1,0,0,1,0,1,0,0],
//  [0,1,0,0,1,1,0,0,1,1,1,0,0],
//  [0,0,0,0,0,0,0,0,0,0,1,0,0],
//  [0,0,0,0,0,0,0,1,1,1,0,0,0],
//  [0,0,0,0,0,0,0,1,1,0,0,0,0]]
// Given the above grid, return 6. Note the answer is not 11, because the island must be connected 4-directionally.

// Example 2:
// [[0,0,0,0,0,0,0,0]]
// Given the above grid, return 0.
// Note: The length of each dimension in the given grid does not exceed 50.

struct Solution;
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid.clone();
        let mut result: i32 = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let tmp = Solution::find(i, j, &mut grid);
                if tmp > result {
                    result = tmp;
                }
            }
        }
        result
    }

    fn find(i: usize, j: usize, grid: &mut Vec<Vec<i32>>) -> i32 {
        let mut tmp = 0;
        if grid[i][j] == 1 {
            // NOTE 将遍历过的节点都赋值为0，之后遍历时就不用再计算该节点了
            grid[i][j] = 0;
            tmp += 1;
            if i > 0 {
                tmp += Solution::find(i - 1, j, grid);
            }
            if i + 1 < grid.len() {
                tmp += Solution::find(i + 1, j, grid);
            }
            if j > 0 {
                tmp += Solution::find(i, j - 1, grid);
            }
            if j + 1 < grid[i].len() {
                tmp += Solution::find(i, j + 1, grid);
            }
        }
        return tmp;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::max_area_of_island(vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
        ]),
        6
    );
    assert_eq!(
        Solution::max_area_of_island(vec![vec![0, 1], vec![1, 1]]),
        3
    );
}
