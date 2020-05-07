// <只出现一次的数字>
// Given a non-empty array of integers, every element appears twice except for one. Find that single one.
// Note:
// Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?

// Example 1:
// Input: [2,2,1]
// Output: 1

// Example 2:
// Input: [4,1,2,1,2]
// Output: 4

// 解题思路
// 方法一：异或运算
//  异或：相同的值异或为0，所以当两两相同时异或的结果就是唯一剩下的数
// 方法二：
//  排序 -> 遍历奇数位和偶数位是否相等，如果不相等则返回奇数位的数
struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        return nums.iter().fold(0, |sum, x| sum ^ x);
    }

    pub fn single_numbe2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut i = 0;
        while i < nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                i += 2;
            } else {
                return nums[i];
            }
        }
        return nums[nums.len() - 1];
    }
}

#[test]
fn run() {
    assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
}
