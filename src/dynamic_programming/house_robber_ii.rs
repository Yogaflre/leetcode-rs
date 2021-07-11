use std::cmp::max;

// <打家劫舍 II>

// You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed. All houses at this place are arranged in a circle. That means the first house is the neighbor of the last one. Meanwhile, adjacent houses have a security system connected, and it will automatically contact the police if two adjacent houses were broken into on the same night.
// Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.

// Example 1:
// Input: nums = [2,3,2]
// Output: 3
// Explanation: You cannot rob house 1 (money = 2) and then rob house 3 (money = 2), because they are adjacent houses.

// Example 2:
// Input: nums = [1,2,3,1]
// Output: 4
// Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
// Total amount you can rob = 1 + 3 = 4.

// Example 3:
// Input: nums = [0]
// Output: 0

// Constraints:
// 1 <= nums.length <= 100
// 0 <= nums[i] <= 1000

struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![-1; nums.len()];
        let mut res = 0;
        for i in 0..nums.len() {
            res = max(res, nums[i] + Self::find(&nums, i + 2, i, &mut dp));
        }
        println!("{:?}", dp);
        return res;
    }

    fn find(nums: &[i32], index: usize, start: usize, dp: &mut [i32]) -> i32 {
        if index >= nums.len() || (index != 0 && index == nums.len() - 1 && start == 0) {
            return 0;
        }
        if dp[index] != -1 {
            return dp[index];
        }
        let mut res = 0;
        for i in index..nums.len() {
            res = max(res, nums[i] + Self::find(nums, i + 2, start, dp));
        }
        dp[index] = res;
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 3);
    assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob(vec![0]), 0);
    assert_eq!(Solution::rob(vec![1]), 1);
}
