// <最大子序和>
// Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.

// Example:
// Input: [-2,1,-3,4,-1,2,1,-5,4],

// Output: 6
// Explanation: [4,-1,2,1] has the largest sum = 6.

// Follow up:
// If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.

// 解题思路
// 方法一：动态规划
//  设置result和tmp两个变量，result用来存储结果集，tmp用来存储子数组大小。从第一个元素开始遍历：
//      当tmp+num[i]>0：代表数组总和是正数，存在变大的可能；如果为负数则可以直接丢弃(必须)
//      tmp>0：代表当前数组和为正数(2选1)
//      当tmp>=nums[i]：代表新元素比tmp整个都大(2选1)
use std::cmp::max;
struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        let mut tmp = nums[0];
        for i in 1..nums.len() {
            // NOTE 这里的判断条件很关键！
            if tmp + nums[i] > 0 && (tmp > 0 || tmp >= nums[i]) {
                tmp += nums[i];
            } else {
                tmp = nums[i];
            }
            result = max(result, tmp);
        }
        return result;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );
    assert_eq!(Solution::max_sub_array(vec![3, -1, 2, -3]), 4);
    assert_eq!(Solution::max_sub_array(vec![3, -3, 2, -3]), 3);
    assert_eq!(Solution::max_sub_array(vec![-2, 1]), 1);
    assert_eq!(Solution::max_sub_array(vec![-2, -1]), -1);
    assert_eq!(Solution::max_sub_array(vec![-2147483647]), -2147483647);
    assert_eq!(Solution::max_sub_array(vec![1, 2]), 3);
}
