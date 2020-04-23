// Given an array nums of n integers and an integer target, find three integers in nums such that the sum is closest to target.
// Return the sum of the three integers. You may assume that each input would have exactly one solution.

// Example:
// Given array nums = [-1, 2, 1, -4], and target = 1.

// The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).

// 解题思路：
// 方法一：暴力搜索(排序+双指针)
//  首先将数组排序，遍历数组0..len-2的索引位置，作为第一个元素。并且随便选取三个元素作为初始值
//  第一个元素选取完成后，取i+1为left左指针，nums.len()为右指针，比较当前三个元素与初始值谁更符合要求，并替换
//  遍历，当前元素绝对值 < target时，left右移；绝对值 > target时，right左移；= target时则返回结果
struct Solution;
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        // NOTE 优先排序！方便后面使用双指针
        nums.sort();
        let mut result = nums[0] + nums[1] + nums[nums.len() - 1];

        for i in 0..nums.len() - 2 {
            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let tmp = nums[i] + nums[left] + nums[right];
                if (tmp - target).abs() < (result - target).abs() {
                    result = tmp;
                }
                let cmp = tmp - target;
                if cmp > 0 {
                    right -= 1;
                } else if cmp < 0 {
                    left += 1;
                } else {
                    return result;
                }
            }
        }
        return result;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::three_sum_closest(vec![1, -3, 0, 3], 0), 0);
    assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, 4], 1), 2);
    assert_eq!(Solution::three_sum_closest(vec![0, 2, 1, -3], 1), 0);
    assert_eq!(Solution::three_sum_closest(vec![1, 1, -1, -1, 3], -1), -1);
    assert_eq!(
        Solution::three_sum_closest(vec![1, 2, 4, 8, 16, 32, 64, 128], 82),
        82
    );
}
