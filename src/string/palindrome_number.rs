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

struct Solution;
impl Solution {
    /*
     * Follow up: 增加临时值y，不断增大y并且减小x。对比x和y是否相同
     */
    pub fn is_palindrome(x: i32) -> bool {
        //负数，非0的10的倍数均不符合条件
        if x < 0 || (x != 0 && x % 10 == 0) {
            return false;
        }
        let mut x = x;
        let mut y = 0;
        while x > y {
            y = y * 10 + (x % 10);
            x /= 10;
        }
        return x == y || x == y / 10;
    }

    /*
     * 将x变为字符串并反转，查看反转是否和原字符想等
     */
    pub fn is_palindrome2(x: i32) -> bool {
        x.to_string() == x.to_string().chars().rev().collect::<String>()
    }
    /*
     * 将x拆分为字符数组，遍历数组是否回文
     */
    pub fn is_palindrome3(x: i32) -> bool {
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
}

#[test]
pub fn run() {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(-121), false);
}
