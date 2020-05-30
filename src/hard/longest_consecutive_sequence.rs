// <最长连续序列>

// Given an unsorted array of integers, find the length of the longest consecutive elements sequence.
// Your algorithm should run in O(n) complexity.

// Example:
// Input: [100, 4, 200, 1, 3, 2]
// Output: 4
// Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.

use std::cmp::max;
struct Solution;
impl Solution {
    /**
     * 排序(不符合题意)
     * 时间复杂度：O(nlogn)
     */
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut nums = nums;
        nums.sort();
        let mut result = 1;
        let mut res = 1;
        let mut tmp = nums[0];
        for n in nums {
            if tmp + 1 == n {
                res += 1;
                result = max(res, result);
            } else if tmp != n {
                res = 1;
            }
            tmp = n;
        }
        return result;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::longest_consecutive(vec![9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6]),
        7
    );
    assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
}
