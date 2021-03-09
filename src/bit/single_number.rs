// <只出现一次的数字>

// Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
// Follow up: Could you implement a solution with a linear runtime complexity and without using extra memory?

// Example 1:
// Input: nums = [2,2,1]
// Output: 1

// Example 2:
// Input: nums = [4,1,2,1,2]
// Output: 4

// Example 3:
// Input: nums = [1]
// Output: 1

// Constraints:
// 1 <= nums.length <= 3 * 104
// -3 * 104 <= nums[i] <= 3 * 104
// Each element in the array appears twice except for one element which appears only once.

struct Solution;
impl Solution {
    /*
     * 异或运算，成对的数字异或都为0，最后余下的数字则只出现一次
     */
    pub fn single_number(nums: Vec<i32>) -> i32 {
        return nums.into_iter().fold(0, |a, b| a ^ b);
    }
}

#[test]
fn run() {
    assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    assert_eq!(Solution::single_number(vec![1]), 1);
}
