// <螺旋矩阵>

// Given a matrix of m x n elements (m rows, n columns), return all elements of the matrix in spiral order.

// Example 1:
// Input:
// [
//  [ 1, 2, 3 ],
//  [ 4, 5, 6 ],
//  [ 7, 8, 9 ]
// ]
// Output: [1,2,3,6,9,8,7,4,5]

// Example 2:
// Input:
// [
//   [1, 2, 3, 4],
//   [5, 6, 7, 8],
//   [9,10,11,12]
// ]
// Output: [1,2,3,4,8,12,11,10,9,5,6,7]

struct Solution;
impl Solution {
    /**
     * 递归
     * 依次递归缩小matrix的范围，递归增加is_rotate属性来判断是否转向。当is_rotate为true时，只取内部list的第一个元素
     * false    true     false  true   false
     * 1 2 3    6 5 4    8 7
     * 4 5 6 -> 9 8 7 -> 5 4 -> 4 5 -> 5
     * 7 8 9
     * 取1 2 3  取6 9    取8 7   取4    取5
     */
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut matrix = matrix;
        let mut result = vec![];
        result.extend(Self::recursive(&mut matrix, false));
        return result;
    }

    fn recursive(matrix: &mut Vec<Vec<i32>>, is_rotate: bool) -> Vec<i32> {
        if matrix.is_empty() {
            return vec![];
        }
        let mut result = vec![];
        if is_rotate {
            let mut i = 0;
            let mut length = matrix.len();
            while i < length {
                matrix[i].reverse();
                result.push(matrix[i][0]);
                matrix[i].remove(0);
                if matrix[i].is_empty() {
                    matrix.remove(i);
                    length -= 1;
                } else {
                    i += 1;
                }
            }
            matrix.reverse();
            result.append(&mut Self::recursive(matrix, false));
        } else {
            result.append(&mut matrix[0]);
            matrix.remove(0);
            result.append(&mut Self::recursive(matrix, true));
        }
        return result;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::spiral_order(vec![vec![1], vec![2], vec![3], vec![4], vec![5], vec![6]]),
        vec![1, 2, 3, 4, 5, 6]
    );
    assert_eq!(
        Solution::spiral_order(vec![vec![7], vec![9], vec![6]]),
        vec![7, 9, 6]
    );
    assert_eq!(
        Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
    );
    assert_eq!(
        Solution::spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12]
        ]),
        vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
    );
}
