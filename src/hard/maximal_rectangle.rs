// <最大矩形>

// Given a 2D binary matrix filled with 0's and 1's, find the largest rectangle containing only 1's and return its area.

// Example:
// Input:
// [
//   ["1","0","1","0","0"],
//   ["1","0","1","1","1"],
//   ["1","1","1","1","1"],
//   ["1","0","0","1","0"]
// ]
// Output: 6

use std::cmp::max;
struct Solution;
impl Solution {
    /**
     * 解题思路
     * 遍历每一个节点，当节点为位置为1时，进入内层循环
     * 内层循环：遍历每个节点，找到最大的长和宽，并更新最大结果
     */
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.len() == 0 {
            return 0;
        }
        let matrix = matrix;
        let mut items: Vec<Vec<i32>> = vec![vec![0; matrix[0].len()]; matrix.len()];
        let mut result = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                result = max(Self::get_rectangle(&matrix, i, j, &mut items), result);
            }
        }
        return result;
    }

    fn get_rectangle(
        matrix: &Vec<Vec<char>>,
        i: usize,
        j: usize,
        items: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if matrix[i][j] == '0' {
            return 0;
        }
        if items[i][j] != 0 {
            return items[i][j];
        }
        let mut tmp = 0;
        let mut l = i;
        let mut w = matrix[0].len() - 1;
        for a in i..matrix.len() {
            if matrix[a][j] == '1' {
                l = a;
                let mut tmp = j;
                for b in j..=w {
                    if matrix[a][b] == '1' {
                        w = b;
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
            tmp = max(tmp, ((l - i + 1) * (w - j + 1)) as i32);
        }
        items[i][j] = tmp;
        return tmp;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::maximal_rectangle(vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0']
        ]),
        6
    );
    assert_eq!(
        Solution::maximal_rectangle(vec![
            vec!['1', '1', '1', '1', '1', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1', '1', '1', '0'],
            vec!['1', '1', '1', '1', '1', '1', '1', '0'],
            vec!['1', '1', '1', '1', '1', '0', '0', '0'],
            vec!['0', '1', '1', '1', '1', '0', '0', '0'],
        ]),
        21
    );
}
