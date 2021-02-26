// <搜索二维矩阵2>

// Write an efficient algorithm that searches for a value in an m x n matrix. This matrix has the following properties:
// Integers in each row are sorted in ascending from left to right.
// Integers in each column are sorted in ascending from top to bottom.

// Example:
// Consider the following matrix:
// [
//   [1,   4,  7, 11, 15],
//   [2,   5,  8, 12, 19],
//   [3,   6,  9, 16, 22],
//   [10, 13, 14, 17, 24],
//   [18, 21, 23, 26, 30]
// ]
// Given target = 5, return true.
// Given target = 20, return false.

struct Solution;
impl Solution {
    /**
     * 从右上方开始搜索矩阵
     * 当前值 = target时，返回true
     * 当前值 < target时，向下移动
     * 当前值 > target时，向左移动
     */
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return false;
        }
        let mut i = 0;
        let mut j = matrix[0].len() - 1;
        while i < matrix.len() {
            if matrix[i][j] == target {
                return true;
            } else if matrix[i][j] < target {
                i += 1;
            } else if j > 0 {
                j -= 1;
            } else {
                return false;
            }
        }
        return false;
    }

    /**
     * 二分查找
     * 遍历外层数组，当target大于第一个元素，小于第二个元素时对内层数组进行二分查找
     * 当外层数组第一个元素大于target时，跳出循环
     */
    pub fn search_matrix2(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() != 0 && matrix[0].len() != 0 {
            let il = matrix.len() - 1;
            let jl = matrix[0].len() - 1;
            for i in 0..=il {
                if matrix[i][0] > target {
                    break;
                }
                if matrix[i][jl] < target {
                    continue;
                }
                let mut js = 0;
                let mut je = jl;
                while js <= je {
                    let mid = js + ((je - js) >> 1);
                    if matrix[i][mid] < target {
                        js = mid + 1;
                    } else if matrix[i][mid] > target {
                        je = mid - 1;
                    } else {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::search_matrix(
            vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30]
            ],
            5
        ),
        true
    );
    assert_eq!(Solution::search_matrix(vec![vec![1, 1]], 0), false);
}
