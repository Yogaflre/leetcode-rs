// <长度最小的子数组>

// Given an array of positive integers nums and a positive integer target, return the minimal length of a contiguous subarray [numsl, numsl+1, ..., numsr-1, numsr] of which the sum is greater than or equal to target. If there is no such subarray, return 0 instead.

// Example 1:
// Input: target = 7, nums = [2,3,1,2,4,3]
// Output: 2
// Explanation: The subarray [4,3] has the minimal length under the problem constraint.

// Example 2:
// Input: target = 4, nums = [1,4,4]
// Output: 1

// Example 3:
// Input: target = 11, nums = [1,1,1,1,1,1,1,1]
// Output: 0

// Constraints:
//     1 <= target <= 109
//     1 <= nums.length <= 105
//     1 <= nums[i] <= 105

// Follow up: If you have figured out the O(n) solution, try coding another solution of which the time complexity is O(n log(n)).

struct Solution;

impl Solution {
    /*
     * 滑动窗口
     */
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        if nums.len() == 1 && nums[0] == target {
            return 1;
        }
        let mut min_length = i32::max_value();
        let mut l = 0;
        let mut r = 0;
        while r < nums.len() && l <= r {
            let tmp: i32 = nums[l..=r].iter().sum();
            if tmp < target {
                r += 1;
            } else if tmp >= target {
                min_length = std::cmp::min(min_length, (r - l + 1) as i32);
                l += 1;
            }
        }
        if min_length == i32::max_value() {
            min_length = 0;
        }
        return min_length;
    }

    /*
     * 二分查找
     */
    pub fn min_sub_array_len_binary_search(target: i32, nums: Vec<i32>) -> i32 {
        let mut pre_counts: Vec<i32> = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            pre_counts[i + 1] = nums[i] + pre_counts[i];
        }
        let mut res: i32 = 0;
        let mut l: i32 = 0;
        loop {
            let r = match pre_counts.binary_search(&(pre_counts[l as usize] + target)) {
                Ok(i) => i as i32,
                Err(i) => i as i32,
            };
            if r as usize >= pre_counts.len() {
                break;
            }
            l = match pre_counts.binary_search(&(pre_counts[r as usize] - target)) {
                Ok(i) => i as i32,
                Err(i) => i as i32 - 1,
            };
            if l < 0 {
                break;
            }
            let tmp = r - l;
            if res == 0 || tmp < res {
                res = tmp;
            }
            l += 1;
        }
        return res;
    }
}
