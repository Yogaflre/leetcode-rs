// <托普利茨矩阵>

// Given an m x n matrix, return true if the matrix is Toeplitz. Otherwise, return false.
// A matrix is Toeplitz if every diagonal from top-left to bottom-right has the same elements.

// Example 1:
// Input: matrix = [[1,2,3,4],
//                  [5,1,2,3],
//                  [9,5,1,2]]
// Output: true
// Explanation:
// In the above grid, the diagonals are:
// "[9]", "[5, 5]", "[1, 1, 1]", "[2, 2, 2]", "[3, 3]", "[4]".
// In each diagonal all elements are the same, so the answer is True.

// Example 2:
// Input: matrix = [[1,2],
//                  [2,2]]
// Output: false
// Explanation:
// The diagonal "[1, 2]" has different elements.

// Constraints:
// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 20
// 0 <= matrix[i][j] <= 99

// Follow up:
// What if the matrix is stored on disk, and the memory is limited such that you can only load at most one row of the matrix into the memory at once?
// What if the matrix is so large that you can only load up a partial row into the memory at once?

struct Solution;
impl Solution {
    /*
     * 找规律
     * 设定pre为一行元素中[0..len-1]
     * 用pre依次比较当前行[1..]，如果有不一致，则说明不符合规则
     */
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        if matrix.is_empty() {
            return false;
        }
        let len = matrix[0].len() - 1;
        let mut pre = &matrix[0][0..len];
        for l in &matrix[1..] {
            if pre.iter().zip(&l[1..]).filter(|(x, y)| x != y).count() > 0 {
                return false;
            } else {
                pre = &l[0..len];
            }
        }
        return true;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::is_toeplitz_matrix(vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]]),
        true
    );
    assert_eq!(
        Solution::is_toeplitz_matrix(vec![vec![1, 2], vec![2, 2]]),
        false
    );
}
