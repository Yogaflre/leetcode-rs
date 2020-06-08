// <最大正方形>

// Given a 2D binary matrix filled with 0's and 1's, find the largest square containing only 1's and return its area.

// Example:
// Input:
// 1 0 1 0 0
// 1 0 1 1 1
// 1 1 1 1 1
// 1 0 0 1 0
// Output: 4

use std::cmp::max;
struct Solution;
impl Solution {
    /**
     * 暴力搜索
     * 遍历所有节点，当节点等于1时，找到该节点的最大正方形
     */
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let mut res = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == '1' {
                    res = max(res, Self::get_square(i, j, &matrix));
                }
            }
        }
        return res;
    }

    fn get_square(i: usize, j: usize, matrix: &Vec<Vec<char>>) -> i32 {
        let mut l = i;
        let mut w = j;
        'outer: while l < matrix.len()
            && w < matrix[0].len()
            && matrix[l][j] == '1'
            && matrix[i][w] == '1'
        {
            for a in i..=l {
                for b in j..=w {
                    if matrix[a][b] == '0' {
                        break 'outer;
                    }
                }
            }
            l += 1;
            w += 1;
        }
        return ((l - i) * (w - j)) as i32;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::maximal_square(vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ]),
        4
    );
    assert_eq!(Solution::maximal_square(vec![vec!['1']]), 1);
}
