// <重塑矩阵>

// In MATLAB, there is a very useful function called 'reshape', which can reshape a matrix into a new one with different size but keep its original data.
// You're given a matrix represented by a two-dimensional array, and two positive integers r and c representing the row number and column number of the wanted reshaped matrix, respectively.
// The reshaped matrix need to be filled with all the elements of the original matrix in the same row-traversing order as they were.
// If the 'reshape' operation with given parameters is possible and legal, output the new reshaped matrix; Otherwise, output the original matrix.

// Example 1:
// Input:
// nums =
// [[1,2],
//  [3,4]]
// r = 1, c = 4
// Output:
// [[1,2,3,4]]
// Explanation:
// The row-traversing of nums is [1,2,3,4]. The new reshaped matrix is a 1 * 4 matrix, fill it row by row by using the previous list.

// Example 2:
// Input:
// nums =
// [[1,2],
//  [3,4]]
// r = 2, c = 4
// Output:
// [[1,2],
//  [3,4]]
// Explanation:
// There is no way to reshape a 2 * 2 matrix to a 2 * 4 matrix. So output the original matrix.

// Note:
// The height and width of the given matrix is in range [1, 100].
// The given r and c are all positive.

struct Solution;
impl Solution {
    /*
     * 遍历
     */
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        // 长度不符合直接返回愿数组
        if (nums.len() * nums[0].len()) as i32 != r * c {
            return nums;
        } else {
            // 使用ri和ci记录新数组的索引位置
            let mut res: Vec<Vec<i32>> = vec![vec![0; c as usize]; r as usize];
            let mut ri: usize = 0;
            let mut ci: usize = 0;
            // 将老数组的元素依次写入新数组
            for i in 0..nums.len() {
                for j in 0..nums[0].len() {
                    if ci >= c as usize {
                        ci = 0;
                        ri += 1;
                    }
                    res[ri][ci] = nums[i][j];
                    ci += 1;
                }
            }
            return res;
        }
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
        vec![vec![1, 2, 3, 4]]
    );
    assert_eq!(
        Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4),
        vec![vec![1, 2], vec![3, 4]]
    );
}
