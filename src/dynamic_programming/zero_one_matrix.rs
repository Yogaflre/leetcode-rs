// <01矩阵>

// Given an m x n binary matrix mat, return the distance of the nearest 0 for each cell.
// The distance between two adjacent cells is 1.

// Example 1:
// Input: mat = [[0,0,0],[0,1,0],[0,0,0]]
// Output: [[0,0,0],[0,1,0],[0,0,0]]

// Example 2:
// Input: mat = [[0,0,0],[0,1,0],[1,1,1]]
// Output: [[0,0,0],[0,1,0],[1,2,1]]

// Constraints:
// m == mat.length
// n == mat[i].length
// 1 <= m, n <= 104
// 1 <= m * n <= 104
// mat[i][j] is either 0 or 1.
// There is at least one 0 in mat.

struct Solution;
impl Solution {
    // FIXME time limit
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![-1; mat[0].len()]; mat.len()];
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                let mut dp = res.clone();
                res[i][j] = Self::recursive(&mut mat, i, j, &mut dp);
            }
        }
        return res;
    }

    pub fn recursive(mat: &mut Vec<Vec<i32>>, i: usize, j: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if dp[i][j] != -1 {
            return dp[i][j];
        }
        if mat[i][j] == 0 {
            dp[i][j] = 0;
            return 0;
        }
        if mat[i][j] == 2 {
            return i32::MAX - 1;
        }
        let mut dist = i32::MAX - 1;
        mat[i][j] = 2;
        if i > 0 {
            dist = std::cmp::min(dist, Self::recursive(mat, i - 1, j, dp));
        }
        if i < mat.len() - 1 {
            dist = std::cmp::min(dist, Self::recursive(mat, i + 1, j, dp));
        }
        if j > 0 {
            dist = std::cmp::min(dist, Self::recursive(mat, i, j - 1, dp));
        }
        if j < mat[0].len() - 1 {
            dist = std::cmp::min(dist, Self::recursive(mat, i, j + 1, dp));
        }
        mat[i][j] = 1;
        dp[i][j] = dist + 1;
        return dp[i][j];
    }
}
