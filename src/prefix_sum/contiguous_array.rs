// <连续数组>

// Given a binary array nums, return the maximum length of a contiguous subarray with an equal number of 0 and 1.

// Example 1:
// Input: nums = [0,1]
// Output: 2
// Explanation: [0, 1] is the longest contiguous subarray with an equal number of 0 and 1.

// Example 2:
// Input: nums = [0,1,0]
// Output: 2
// Explanation: [0, 1] (or [1, 0]) is a longest contiguous subarray with equal number of 0 and 1.

// Constraints:
// 1 <= nums.length <= 105
// nums[i] is either 0 or 1.

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut len = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, -1);

        let mut tmp = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                tmp += -1;
            } else {
                tmp += 1;
            }
            if let Some(j) = map.get(&tmp) {
                len = std::cmp::max(len, i as i32 - j);
            } else {
                map.insert(tmp, i as i32);
            }
        }
        return len;
    }
}
