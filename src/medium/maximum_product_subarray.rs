// <乘积最大子数组>

// Given an integer array nums, find the contiguous subarray within an array (containing at least one number) which has the largest product.

// Example 1:
// Input: [2,3,-2,4]
// Output: 6
// Explanation: [2,3] has the largest product 6.

// Example 2:
// Input: [-2,0,-1]
// Output: 0
// Explanation: The result cannot be 2, because [-2,-1] is not a subarray.

// 解题思路
// 方法一：
//  记录当前最大最小值和结果最大值，遍历
//      当n>=0时，记录当前最大最小值
//      当n<0时，用当前最大值*负数可能为最小值，用当前最小值*负数可能为最大值
//      最后比较当前最大值与结果最大值
use std::cmp::{max, min};
struct Solution;
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut max_value = nums[0];
        let mut min_value = nums[0];
        let mut max_result = nums[0];
        for n in nums[1..].iter() {
            if *n >= 0 {
                max_value = max(max_value * n, *n);
                min_value = min(min_value * n, *n);
            } else {
                let tmp = max_value;
                max_value = max(min_value * n, *n);
                min_value = min(tmp * n, *n);
            }
            max_result = max(max_result, max_value);
        }
        return max_result;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::max_product(vec![3, -1, 4]), 4);
    assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
    assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    assert_eq!(Solution::max_product(vec![1, 2, 0, 5, -1, -2]), 10);
    assert_eq!(Solution::max_product(vec![2, 3, 0, 2, -1, -4]), 8);
    assert_eq!(Solution::max_product(vec![0, 2]), 2);
}
