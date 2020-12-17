// <最长回文串>

// Given a string s which consists of lowercase or uppercase letters, return the length of the longest palindrome that can be built with those letters.
// Letters are case sensitive, for example, "Aa" is not considered a palindrome here.

// Example 1:
// Input: s = "abccccdd"
// Output: 7
// Explanation:
// One longest palindrome that can be built is "dccaccd", whose length is 7.

// Example 2:
// Input: s = "a"
// Output: 1

// Example 3:
// Input: s = "bb"
// Output: 2

// Constraints:
// 1 <= s.length <= 2000
// s consits of lower-case and/or upper-case English letters only.

use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut res = 0;
        // NOTE 使用hash记录字符出现的次数（一次添加，第二次删除）
        let mut set = HashSet::new();
        for c in s.chars() {
            if set.contains(&c) {
                set.remove(&c);
                res += 1;
            } else {
                set.insert(c);
            }
        }
        // 如果set为空说明每个字符都是偶数倍，可回文；如果非空，说明有多余的字符，则在res*2的基础上再+1（当作回文最中间的字符）
        if set.is_empty() {
            return res * 2;
        } else {
            return res * 2 + 1;
        }
    }
}

#[test]
fn run() {
    assert_eq!(Solution::longest_palindrome(String::from("abccccdd")), 7);
    assert_eq!(Solution::longest_palindrome(String::from("a")), 1);
    assert_eq!(Solution::longest_palindrome(String::from("bb")), 2);
}
