// <跳跃游戏>
// Given an array of non-negative integers, you are initially positioned at the first index of the array.
// Each element in the array represents your maximum jump length at that position.
// Determine if you are able to reach the last index.

// Example 1:
// Input: [2,3,1,1,4]
// Output: true
// Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.

// Example 2:
// Input: [3,2,1,0,4]
// Output: false

// Explanation: You will always arrive at index 3 no matter what. Its maximum
//              jump length is 0, which makes it impossible to reach the last index.

// 解题思路
// 方法一：贪心算法
//  从后往前遍历，选取最后一个节点为终点(可变)
//  当遍历到的节点值大于终点位置时，代表该节点一定能到达终点。然后把该节点设置为新的终节点
//  当遍历完成后终结点位置为0时，代表可以从队首跳跃到队尾
// 方法二：回溯法（很慢！）
//  从第一个节点开始，对可以跳跃的节点进行递归，直到找到可跳跃到的末尾节点为止

use std::cmp::min;
struct Solution;
impl Solution {
    // NOTE 转换思维很重要，从后往前遍历
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut last_index = nums.len() - 1;
        for i in (0..nums.len()).rev() {
            if i + nums[i] as usize >= last_index {
                last_index = i;
            }
        }
        return last_index == 0;
    }

    pub fn can_jump2(nums: Vec<i32>) -> bool {
        return Solution::jump2(&nums, 0);
    }

    fn jump2(nums: &Vec<i32>, start: usize) -> bool {
        if start == nums.len() - 1 {
            return true;
        }
        let next = min(start + nums[start] as usize, nums.len() - 1);
        for i in (start + 1..=next).rev() {
            if Solution::jump2(nums, i) {
                return true;
            }
        }
        return false;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::can_jump(vec![1, 2, 3]), true);
    assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
}
