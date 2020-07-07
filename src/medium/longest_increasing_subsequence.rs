// <最长上升子序列>

// Given an unsorted array of integers, find the length of longest increasing subsequence.

// Example:
// Input: [10,9,2,5,3,7,101,18]
// Output: 4
// Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.

// Note:
// There may be more than one LIS combination, it is only necessary for you to return the length.
// Your algorithm should run in O(n2) complexity.
// Follow up: Could you improve it to O(n log n) time complexity?

use std::cmp::max;
struct Solution;
impl Solution {
    /**
     * dp + 二分查找
     * 初始化dp数组，用来存放当前的最长上升子序列。遍历数组
     * 如果当前元素大于上升自序列的最大元素，则直接插入dp
     * 如果当前元素小于最大值，则在dp中(二分)查找到需要插入的位置(总是保留最小值，才能达到最长上升子序列)
     */
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut dp = vec![];
        for n in &nums {
            if dp.is_empty() || n > dp[dp.len() - 1] {
                dp.push(n);
            } else {
                let index = match dp.binary_search(&n) {
                    Ok(i) => i,
                    Err(i) => i,
                };
                dp[index] = n;
            }
        }
        return dp.len() as i32;
    }

    /**
     * 动态规划
     * 创建dp[]，用于持久化每个位置元素的最长上升子序列长度
     * 外层循环遍历所有元素，内层循环遍历0-当前元素。并记录dp和最大值
     */
    pub fn length_of_lis2(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut dp = vec![0; nums.len()];
        dp[0] = 1;
        let mut res = 1;
        for i in 1..nums.len() {
            let mut tmp = 0;
            for j in 0..i {
                if nums[j] < nums[i] {
                    tmp = max(tmp, dp[j]);
                }
            }
            dp[i] = tmp + 1;
            res = max(res, dp[i]);
        }
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::length_of_lis(vec![3, 1, 4]), 2);
    assert_eq!(Solution::length_of_lis(vec![4, 10, 4]), 2);
    assert_eq!(Solution::length_of_lis(vec![1, 3, 6, 7, 9, 4, 10, 5, 6]), 6);
    assert_eq!(Solution::length_of_lis(vec![4, 10, 4, 3, 8, 9]), 3);
    assert_eq!(Solution::length_of_lis(vec![1, 10, 3, 11]), 3);
    assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    assert_eq!(Solution::length_of_lis(vec![2, 15, 3, 7, 8, 6, 18]), 5);
}
