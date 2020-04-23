// <翻转整型>
// Given a 32-bit signed integer, reverse digits of an integer.

// Example 1:
// Input: 123
// Output: 321

// Example 2:
// Input: -123
// Output: -321

// Example 3:
// Input: 120
// Output: 21

// Note:
// Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−231,  231 − 1].
// For the purpose of this problem, assume that your function returns 0 when the reversed integer overflows.

// 解题思路
// 方法一：
//  将input转换为i64的绝对值，并记录正负、将input:i64转换为Vec<char>、对每个数字乘以对应位数构造最终结果、对最终值赋予正负
use std::i32::MAX;
use std::i32::MIN;
struct Solution;
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let flag: bool = x > 0;
        let x: i64 = (x as i64).abs();
        let mut temp: i64 = 1;
        let mut result: i64 = 0;
        for i in x.to_string().chars() {
            result += i.to_digit(10).unwrap() as i64 * temp;
            temp *= 10;
        }
        result = if flag { result } else { result * -1 };
        return if result > MAX as i64 || result < MIN as i64 {
            0
        } else {
            result as i32
        };
    }
}

#[test]
pub fn run() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(-2147483648), 0);
}
