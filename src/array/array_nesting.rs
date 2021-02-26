// <数组嵌套>

// A zero-indexed array A of length N contains all integers from 0 to N-1. Find and return the longest length of set S, where S[i] = {A[i], A[A[i]], A[A[A[i]]], ... } subjected to the rule below.
// Suppose the first element in S starts with the selection of element A[i] of index = i, the next element in S should be A[A[i]], and then A[A[A[i]]]… By that analogy, we stop adding right before a duplicate element occurs in S.

// Example 1:
// Input: A = [5,4,0,3,1,6,2]
// Output: 4
// Explanation:
// A[0] = 5, A[1] = 4, A[2] = 0, A[3] = 3, A[4] = 1, A[5] = 6, A[6] = 2.
// One of the longest S[K]:
// S[0] = {A[0], A[5], A[6], A[2]} = {5, 6, 2, 0}

// Note:
// N is an integer within the range [1, 20,000].
// The elements of A are all distinct.
// Each element of A is an integer within the range [0, N-1].

use std::cmp::max;

struct Solution;
impl Solution {
    /*
     * 降遍历后的数字做标记，防止被再次遍历(每个位置的元素都会走过相同的路径)
     */
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut res = 0;
        for i in 0..nums.len() {
            let mut tmp = 0;
            let mut index = nums[i];
            while index != -1 {
                tmp += 1;
                let i = nums[index as usize];
                // 将访问过的位置赋为-1
                nums[index as usize] = -1;
                index = i;
            }
            res = max(res, tmp);
        }
        // 因为会把结尾和开头重复的元素多计算一遍，所以最后-1
        return res - 1;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]), 4);
    assert_eq!(Solution::array_nesting(vec![0, 1, 2]), 1);
}
