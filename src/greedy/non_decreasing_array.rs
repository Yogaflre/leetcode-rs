// <非递减数列>

// Given an array nums with n integers, your task is to check if it could become non-decreasing by modifying at most 1 element.
// We define an array is non-decreasing if nums[i] <= nums[i + 1] holds for every i (0-based) such that (0 <= i <= n - 2).

// Example 1:
// Input: nums = [4,2,3]
// Output: true
// Explanation: You could modify the first 4 to 1 to get a non-decreasing array.

// Example 2:
// Input: nums = [4,2,1]
// Output: false
// Explanation: You can't get a non-decreasing array by modify at most one element.

// Constraints:
// 1 <= n <= 10 ^ 4
// - 10 ^ 5 <= nums[i] <= 10 ^ 5

struct Solution;
impl Solution {
    /**
     * 贪心
     * 依次遍历数组，比较当前元素和前一个元素
     * 当前一个元素大时，需要判断替换哪个元素
     *  当i-2的元素大于当前元素时，将i位置替换为i-1 (因为i-2肯定小于i-1，所以将i设置为i-1，保证递增)
     *  当i-2的元素小于等于当前元素时，将i-1位置替换为i (将i-1设置为比i-2大，并且和i相等的值)
     */
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        let mut change = 0;
        for i in 1..nums.len() {
            if nums[i - 1] > nums[i] {
                if i >= 2 && nums[i - 2] > nums[i] {
                    nums[i] = nums[i - 1];
                } else {
                    nums[i - 1] = nums[i];
                }
                change += 1;
            }
        }
        return change <= 1;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::check_possibility(vec![4, 2, 3]), true);
    assert_eq!(Solution::check_possibility(vec![4, 2, 1]), false);
    assert_eq!(Solution::check_possibility(vec![3, 4, 2, 3]), false);
    assert_eq!(Solution::check_possibility(vec![5, 7, 1, 8]), true);
}
