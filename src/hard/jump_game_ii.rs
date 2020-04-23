// <跳跃游戏2>
// Given an array of non-negative integers, you are initially positioned at the first index of the array.
// Each element in the array represents your maximum jump length at that position.
// Your goal is to reach the last index in the minimum number of jumps.

// Example:
// Input: [2,3,1,1,4]
// Output: 2
// Explanation: The minimum number of jumps to reach the last index is 2.
//     Jump 1 step from index 0 to 1, then 3 steps to the last index.

// 解题思路
// 方法一：贪心
//  从后往前遍历(起点index为len-1)，每次寻找最大可达路径(变更index位置)，如果路径有变则将jump数+1。直到index<=0为止
// 方法二：动态规划(不够巧妙)
//  正向遍历，逐步缩减问题规模
struct Solution;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut index = nums.len() - 1;
        let mut result = 0;
        while index > 0 {
            let flag = index;
            for i in (0..index).rev() {
                if nums[i] as usize >= flag - i {
                    index = i;
                }
            }
            if flag != index {
                result += 1;
            }
        }
        return result;
    }

    pub fn jump2(nums: Vec<i32>) -> i32 {
        let mut items: Vec<i32> = vec![-1; nums.len()];
        return Self::jump_recursive(&nums, 0, &mut items);
    }

    fn jump_recursive(nums: &Vec<i32>, index: usize, items: &mut Vec<i32>) -> i32 {
        if index == nums.len() - 1 {
            return 0;
        }
        if index >= nums.len() {
            return -1;
        }
        if items[index] != -1 {
            return items[index];
        }
        let mut min_tmp = -1;
        for i in 1..=nums[index] {
            let tmp = Self::jump_recursive(nums, index + i as usize, items);
            if tmp != -1 && (min_tmp == -1 || tmp < min_tmp) {
                min_tmp = tmp;
            }
        }
        if min_tmp != -1 {
            min_tmp += 1;
        }
        items[index] = min_tmp;
        return min_tmp;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    assert_eq!(Solution::jump(vec![1, 2]), 1);
    assert_eq!(Solution::jump(vec![1, 2, 3]), 2);
}
