// <字符串相加>

// Given two non-negative integers num1 and num2 represented as string, return the sum of num1 and num2.

// Note:
// The length of both num1 and num2 is < 5100.
// Both num1 and num2 contains only digits 0-9.
// Both num1 and num2 does not contain any leading zero.
// You must not use any built-in BigInteger library or convert the inputs to integer directly.

struct Solution;
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let num1: Vec<char> = num1.chars().collect();
        let num2: Vec<char> = num2.chars().collect();
        let mut res = String::new();
        let mut i = (num1.len() - 1) as i32;
        let mut j = (num2.len() - 1) as i32;
        let mut add = 0;
        while i >= 0 || j >= 0 || add != 0 {
            let a = if i < 0 {
                0
            } else {
                num1[i as usize].to_digit(10).unwrap()
            };
            let b = if j < 0 {
                0
            } else {
                num2[j as usize].to_digit(10).unwrap()
            };
            let sum = a + b + add;
            add = if sum > 9 { 1 } else { 0 };
            res.push_str(&(sum % 10).to_string());
            i -= 1;
            j -= 1;
        }
        return res.chars().rev().collect::<String>();
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::add_strings("9".to_string(), "12".to_string()),
        "21".to_string()
    );
}
