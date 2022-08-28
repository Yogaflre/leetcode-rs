// <第N位数字>

// Given an integer n, return the nth digit of the infinite integer sequence [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ...].

// Example 1:
// Input: n = 3
// Output: 3

// Example 2:
// Input: n = 11
// Output: 0
// Explanation: The 11th digit of the sequence 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ... is a 0, which is part of the number 10.

// Constraints:
// 1 <= n <= 231 - 1

struct Solution;

impl Solution {
    // FIXME
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut s = String::new();
        for i in 1..=n {
            s.push_str(&i.to_string());
        }
        return s.chars().collect::<Vec<char>>()[n as usize - 1]
            .to_digit(10)
            .unwrap() as i32;
    }
}
