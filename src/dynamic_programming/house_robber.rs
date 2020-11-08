// <打家劫舍>

// You are a professional robber planning to rob houses along a street.
// Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security system connected and it will automatically contact the police if two adjacent houses were broken into on the same night.
// Given a list of non-negative integers representing the amount of money of each house, determine the maximum amount of money you can rob tonight without alerting the police.

// Example 1:
// Input: [1,2,3,1]
// Output: 4
// Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
//              Total amount you can rob = 1 + 3 = 4.

// Example 2:
// Input: [2,7,9,3,1]
// Output: 12
// Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
//              Total amount you can rob = 2 + 9 + 1 = 12.

use std::cmp::max;
struct Solution;
impl Solution {
    /**
     * 动态规划
     * 遍历所有节点，并且递归起始位置为该节点索引位置+2的节点。取最大值
     * 使用items来保存每个起始位置的最大结果
     */
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut items = vec![-1; nums.len()];
        return Self::recursive(&nums, 0, &mut items);
    }

    fn recursive(nums: &Vec<i32>, index: usize, items: &mut Vec<i32>) -> i32 {
        if index >= nums.len() {
            return 0;
        }
        if items[index] != -1 {
            return items[index];
        }
        let mut res = 0;
        for i in index..nums.len() {
            res = max(res, nums[i] + Self::recursive(nums, i + 2, items));
        }
        items[index] = res;
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    assert_eq!(Solution::rob(vec![1]), 1);
    assert_eq!(Solution::rob(vec![1, 2]), 2);
}
