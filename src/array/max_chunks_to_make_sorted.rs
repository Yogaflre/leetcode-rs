// <最多能完成排序的块>

// Given an array arr that is a permutation of [0, 1, ..., arr.length - 1], we split the array into some number of "chunks" (partitions), and individually sort each chunk.  After concatenating them, the result equals the sorted array.
// What is the most number of chunks we could have made?

// Example 1:
// Input: arr = [4,3,2,1,0]
// Output: 1
// Explanation:
// Splitting into two or more chunks will not return the required result.
// For example, splitting into [4, 3], [2, 1, 0] will result in [3, 4, 0, 1, 2], which isn't sorted.

// Example 2:
// Input: arr = [1,0,2,3,4]
// Output: 4
// Explanation:
// We can split into two chunks, such as [1, 0], [2, 3, 4].
// However, splitting into [1, 0], [2], [3], [4] is the highest number of chunks possible.

// Note:
// arr will have length in range [1, 10].
// arr[i] will be a permutation of [0, 1, ..., arr.length - 1].

use std::cmp::max;
struct Solution;
impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        if arr.is_empty() {
            return 0;
        }
        let mut res = 0;
        let mut r = arr[0];
        for (i, n) in arr.into_iter().enumerate() {
            r = max(r, n);
            // NOTE 一组数据按顺序排列后最大值应该等于当前的索引位置。所以这组数据可连续，则进行分割
            if r == i as i32 {
                res += 1;
            }
        }
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
    assert_eq!(Solution::max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
    assert_eq!(Solution::max_chunks_to_sorted(vec![1, 2, 0, 3]), 2);
}
