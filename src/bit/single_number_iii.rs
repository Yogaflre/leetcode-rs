// <只出现一次的数字 III>

// Given an integer array nums, in which exactly two elements appear only once and all the other elements appear exactly twice. Find the two elements that appear only once. You can return the answer in any order.
// Follow up: Your algorithm should run in linear runtime complexity. Could you implement it using only constant space complexity?

// Example 1:
// Input: nums = [1,2,1,3,2,5]
// Output: [3,5]
// Explanation:  [5, 3] is also a valid answer.

// Example 2:
// Input: nums = [-1,0]
// Output: [-1,0]

// Example 3:
// Input: nums = [0,1]
// Output: [1,0]

// Constraints:
// 2 <= nums.length <= 3 * 104
// -231 <= nums[i] <= 231 - 1
// Each integer in nums will appear twice, only two integers will appear once.

struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0, 0];
        // 计算两个只出现一次数字的异或
        let mut diff = 0;
        for n in nums.iter() {
            diff ^= *n;
        }
        // 根据异或结果找到默认一个二进制不同的位置(默认找第一个)。负数是对原数按位去反+1，所以“&”刚好可以取到不同的第一位
        diff &= -diff;
        // NOTE 依次将diff & n的不同结果异或到不同的位置
        for n in nums.into_iter() {
            if diff & n == 0 {
                res[0] ^= n;
            } else {
                res[1] ^= n;
            }
        }
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::single_number(vec![1, 2, 1, 3, 2, 5]), vec![5, 3]);
    assert_eq!(Solution::single_number(vec![-1, 0]), vec![0, -1]);
    assert_eq!(Solution::single_number(vec![0, 1]), vec![0, 1]);
}
