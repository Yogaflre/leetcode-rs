// <有序矩阵中第K小的元素>

// Given a n x n matrix where each of the rows and columns are sorted in ascending order, find the kth smallest element in the matrix.
// Note that it is the kth smallest element in the sorted order, not the kth distinct element.

// Example:
// matrix = [
//    [ 1,  5,  9],
//    [10, 11, 13],
//    [12, 13, 15]
// ],
// k = 8,
// return 13.

// Note:
// You may assume k is always valid, 1 ≤ k ≤ n2.

use std::collections::BinaryHeap;
struct Solution;
impl Solution {
    /*
     * 二分查找
     * TODO
     */
    pub fn kth_smallest_binary_search(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        0
    }

    /*
     * 最大堆
     */
    pub fn kth_smallest_heap(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut heap = BinaryHeap::with_capacity(k as usize);
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                heap.push(matrix[i][j]);
                // NOTE 依次将元素插入最大堆中。如果堆大小超过k，则把当前最大值出堆（保证当前第k大的值一直在堆中）
                if heap.len() > k as usize {
                    heap.pop();
                }
            }
        }
        return heap.pop().unwrap();
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::kth_smallest_binary_search(vec![vec![1, 2], vec![3, 4]], 3),
        3
    );
    assert_eq!(
        Solution::kth_smallest_heap(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8),
        13
    );
}
