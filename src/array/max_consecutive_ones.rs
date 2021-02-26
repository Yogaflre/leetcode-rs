// <最大连续1的个数>

// Given a binary array, find the maximum number of consecutive 1s in this array.
//
// Example 1:
// Input: [1,1,0,1,1,1]
// Output: 3
// Explanation: The first two digits or the last three digits are consecutive 1s.
//     The maximum number of consecutive 1s is 3.

// Note:
// The input array will only contain 0 and 1.
// The length of input array is a positive integer and will not exceed 10,000

use std::cmp::max;
struct Solution;
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut tmp = 0;
        for n in nums.into_iter() {
            if n == 1 {
                tmp += 1;
            } else {
                count = max(count, tmp);
                tmp = 0;
            }
        }
        return max(count, tmp);
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
        3
    );
}
