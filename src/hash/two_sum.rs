// <两数之和>

// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.

// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Output: Because nums[0] + nums[1] == 9, we return [0, 1].

// Example 2:
// Input: nums = [3,2,4], target = 6
// Output: [1,2]

// Example 3:
// Input: nums = [3,3], target = 6
// Output: [0,1]

// Constraints:
// 2 <= nums.length <= 103
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// Only one valid answer exists.

use std::collections::HashMap;
struct Solution;
impl Solution {
    /*
     * HashMap
     */
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            if let Some(j) = map.get(&(target - v)) {
                return vec![i as i32, *j as i32];
            } else {
                map.insert(*v, i);
            }
        }
        return vec![];
    }

    /*
     * 排序+双指针
     */
    pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // NOTE 记录排序前的索引位置
        let mut tuple: Vec<(i32, usize)> = vec![];
        for (i, v) in nums.iter().enumerate() {
            tuple.push((*v, i));
        }
        tuple.sort_by_key(|t| t.0);
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            let sum = tuple[l].0 + tuple[r].0;
            if sum == target {
                return vec![tuple[l].1 as i32, tuple[r].1 as i32];
            } else if sum < target {
                l += 1;
            } else {
                r -= 1;
            }
        }
        return vec![];
    }
}

#[test]
fn run() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 0]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![2, 1]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![1, 0]);
}
