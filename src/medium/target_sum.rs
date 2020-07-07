// <目标和>

// You are given a list of non-negative integers, a1, a2, ..., an, and a target, S. Now you have 2 symbols + and -. For each integer, you should choose one from + and - as its new symbol.
// Find out how many ways to assign symbols to make sum of integers equal to target S.

// Example 1:
// Input: nums is [1, 1, 1, 1, 1], S is 3.
// Output: 5
// Explanation:
// -1+1+1+1+1 = 3
// +1-1+1+1+1 = 3
// +1+1-1+1+1 = 3
// +1+1+1-1+1 = 3
// +1+1+1+1-1 = 3
// There are 5 ways to assign symbols to make the sum of nums be target 3.

// Constraints:
// The length of the given array is positive and will not exceed 20.
// The sum of elements in the given array will not exceed 1000.
// Your output answer is guaranteed to be fitted in a 32-bit integer.

use std::collections::HashMap;
struct Solution;
impl Solution {
    /**
     * 动态规划
     * 递归，当索引位置遍历结束后，判断值是否满足提议，满足则返回1计数，不满足则返回0
     * 将结果集通过map进行持久化
     */
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        let mut dp: HashMap<(usize, i32), i32> = HashMap::new();
        return Self::recursive(&nums, 0, s, 0, &mut dp);
    }

    pub fn recursive(
        nums: &Vec<i32>,
        index: usize,
        s: i32,
        t: i32,
        dp: &mut HashMap<(usize, i32), i32>,
    ) -> i32 {
        if dp.contains_key(&(index, t)) {
            return *dp.get(&(index, t)).unwrap();
        }
        if index == nums.len() {
            if s == t {
                return 1;
            } else {
                return 0;
            }
        }
        let tmp = Self::recursive(nums, index + 1, s, t + nums[index], dp)
            + Self::recursive(nums, index + 1, s, t - nums[index], dp);
        dp.insert((index, t), tmp);
        return tmp;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::find_target_sum_ways(vec![1, 0], 1), 2);
    assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
}
