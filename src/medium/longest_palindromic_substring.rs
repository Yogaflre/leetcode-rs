// <最长回文子串>
// Given a string s, find the longest palindromic substring in s.
// You may assume that the maximum length of s is 1000.

// Example 1:
// Input: "babad"
// Output: "bab"
// Note: "aba" is also a valid answer.

// Example 2:
// Input: "cbbd"
// Output: "bb"

// 解题思路
// 方法一：中心点扩散   O(n^2)
//  遍历所有字符，根据每一个字符向两边扩散寻找到最大回文数
//  （**注意在循环中**）需要同时计算 单个字符和相邻字符扩展，在比较这两者扩展的结果大小，再和总的结果比较
// 方法二：动态规划
struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        if len < 2 {
            return s;
        }
        let mut result: String = String::from("");
        for i in 0..len - 1 {
            let temp1: String = Self::check(&chars, i as i32, i as i32);
            let temp2: String = Self::check(&chars, i as i32, (i + 1) as i32);
            if temp1.len() > temp2.len() {
                if temp1.len() > result.len() {
                    result = temp1;
                }
            } else {
                if temp2.len() > result.len() {
                    result = temp2;
                }
            }
        }
        return result;
    }

    fn check(chars: &Vec<char>, mut l: i32, mut r: i32) -> String {
        while l >= 0 && r < chars.len() as i32 {
            if chars[l as usize] == chars[r as usize] {
                l = l - 1;
                r = r + 1;
            } else {
                break;
            }
        }
        return (&chars[(l + 1) as usize..r as usize]).into_iter().collect();
    }
}

#[test]
fn run() {
    assert_eq!(Solution::longest_palindrome(String::from("babad")), "bab");
    assert_eq!(Solution::longest_palindrome(String::from("cbbd")), "bb");
    assert_eq!(Solution::longest_palindrome(String::from("eabcb")), "bcb");
    assert_eq!(Solution::longest_palindrome(String::from("bbbb")), "bbbb");
    assert_eq!(Solution::longest_palindrome(String::from("aba")), "aba");
}
