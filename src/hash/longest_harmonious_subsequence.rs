// <最长和谐子序列>

// We define a harmonious array as an array where the difference between its maximum value and its minimum value is exactly 1.
// Given an integer array nums, return the length of its longest harmonious subsequence among all its possible subsequences.
// A subsequence of array is a sequence that can be derived from the array by deleting some or no elements without changing the order of the remaining elements.

// Example 1:
// Input: nums = [1,3,2,2,5,2,3,7]
// Output: 5
// Explanation: The longest harmonious subsequence is [3,2,2,2,3].

// Example 2:
// Input: nums = [1,2,3,4]
// Output: 2

// Example 3:
// Input: nums = [1,1,1,1]
// Output: 0

// Constraints:
// 1 <= nums.length <= 2 * 104
// -109 <= nums[i] <= 109

use std::cmp::max;
use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        // count every number
        let mut map: HashMap<i32, usize> = HashMap::new();
        for n in nums.iter() {
            match map.get_mut(n) {
                Some(v) => *v += 1,
                None => {
                    map.insert(*n, 1);
                }
            }
        }
        // caculate key and key+1 count.
        let mut res = 0;
        for (k, v) in map.iter() {
            if let Some(v) = map.get(&(k + 1)) {
                res = max(res, map.get(&k).unwrap() + v);
            }
        }
        return res as i32;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
    assert_eq!(Solution::find_lhs(vec![1, 2, 3, 4]), 2);
    assert_eq!(Solution::find_lhs(vec![1, 1, 1, 1]), 0);
}
