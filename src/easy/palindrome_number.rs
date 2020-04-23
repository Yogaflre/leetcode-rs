// <回文数>
// Determine whether an integer is a palindrome.
// An integer is a palindrome when it reads the same backward as forward.

// Example 1
// Input: 121
// Output: true

// Example 2:
// Input: -121
// Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

// Example 3:
// Input: 10
// Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

// Follow up:
// Coud you solve it without converting the integer to a string?

// 解题思路
// 方法一：
//  将x拆分为字符数组，遍历数组判断是否为回文
// 方法二：
//  将x转换为字符串并反转，比较两个字符串是否相等
// 方法三（不使用字符串）：
//
struct Solution;
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let nums: Vec<char> = x.to_string().chars().collect();
        let max_index: usize = nums.len() - 1;
        for i in 0..nums.len() / 2 {
            if nums[i] != nums[max_index - i] {
                return false;
            }
        }
        return true;
    }

    pub fn is_palindrome2(x: i32) -> bool {
        x.to_string() == x.to_string().chars().rev().collect::<String>()
    }
}

#[test]
pub fn run() {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(-121), false);

    assert_eq!(Solution::is_palindrome2(121), true);
    assert_eq!(Solution::is_palindrome2(-121), false);
}
