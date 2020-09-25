// <判断子序列>

// Given a string s and a string t, check if s is subsequence of t.
// A subsequence of a string is a new string which is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (ie, "ace" is a subsequence of "abcde" while "aec" is not).

// Follow up:
// If there are lots of incoming S, say S1, S2, ... , Sk where k >= 1B, and you want to check one by one to see if T has its subsequence. In this scenario, how would you change your code?
// Credits:
// Special thanks to @pbrother for adding this problem and creating all test cases.

// Example 1:
// Input: s = "abc", t = "ahbgdc"
// Output: true

// Example 2:
// Input: s = "axc", t = "ahbgdc"
// Output: false

// Constraints:
// 0 <= s.length <= 100
// 0 <= t.length <= 10^4
// Both strings consists only of lowercase characters.

struct Solution;
impl Solution {
    /**
     * 双指针
     * 依次对比两个字符串的元素，直到一方结束
     */
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let mut i = 0;
        let mut j = 0;
        while i < s.len() && j < t.len() {
            if s[i] == t[j] {
                i += 1;
            }
            j += 1;
        }
        return i == s.len();
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::is_subsequence("abc".into(), "ahbgdc".into()),
        true
    );
    assert_eq!(
        Solution::is_subsequence("axc".into(), "ahbgdc".into()),
        false
    );
}
