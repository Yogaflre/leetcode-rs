// <通配符匹配>

// Given an input string (s) and a pattern (p), implement wildcard pattern matching with support for '?' and '*'.
// '?' Matches any single character.
// '*' Matches any sequence of characters (including the empty sequence).
// The matching should cover the entire input string (not partial).

// Note:
// s could be empty and contains only lowercase letters a-z.
// p could be empty and contains only lowercase letters a-z, and characters like ? or *.

// Example 1:
// Input:
// s = "aa"
// p = "a"
// Output: false
// Explanation: "a" does not match the entire string "aa".

// Example 2:
// Input:
// s = "aa"
// p = "*"
// Output: true
// Explanation: '*' matches any sequence.

// Example 3:
// Input:
// s = "cb"
// p = "?a"
// Output: false
// Explanation: '?' matches 'c', but the second letter is 'a', which does not match 'b'.

// Example 4:
// Input:
// s = "adceb"
// p = "*a*b"
// Output: true
// Explanation: The first '*' matches the empty sequence, while the second '*' matches the substring "dce".

// Example 5:
// Input:
// s = "acdcb"
// p = "a*c?b"
// Output: false

struct Solution;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut dp: Vec<Vec<bool>> = vec![vec![false; p.len() + 1]; s.len() + 1];
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        dp[0][0] = true;
        // 过滤p字符串中的*
        for i in 1..=p.len() {
            if p[i - 1] == '*' {
                dp[0][i] = true;
            } else {
                break;
            }
        }
        // 遍历
        for i in 1..=s.len() {
            for j in 1..=p.len() {
                if p[j - 1] == '*' {
                    dp[i][j] = dp[i - 1][j] || dp[i][j - 1];
                } else {
                    dp[i][j] = dp[i - 1][j - 1] && (s[i - 1] == p[j - 1] || p[j - 1] == '?');
                }
            }
        }
        return dp[s.len()][p.len()];
    }

    /**
     * 动态规划(递归)
     * 依次比较对应字符，考虑所有情况
     */
    pub fn is_match2(s: String, p: String) -> bool {
        let mut dp: Vec<Vec<u8>> = vec![vec![0; p.len()]; s.len()];
        return Solution::recursive_match(
            &s.chars().collect::<Vec<char>>(),
            0,
            &p.chars().collect::<Vec<char>>(),
            0,
            &mut dp,
        );
    }

    fn recursive_match(
        s: &[char],
        si: usize,
        p: &[char],
        pi: usize,
        dp: &mut Vec<Vec<u8>>,
    ) -> bool {
        let mut result = false;
        if si == s.len() && pi == p.len() {
            return true;
        } else if si == s.len() || pi == p.len() {
            if pi < p.len() && p[pi] == '*' {
                return Self::recursive_match(s, si, p, pi + 1, dp);
            } else {
                return false;
            }
        }
        if dp[si][pi] != 0 {
            if dp[si][pi] == 1 {
                return true;
            } else {
                return false;
            }
        }
        if s[si] == p[pi] || p[pi] == '?' {
            result = Self::recursive_match(s, si + 1, p, pi + 1, dp);
        } else if p[pi] == '*' {
            result = Self::recursive_match(s, si + 1, p, pi + 1, dp)
                || Self::recursive_match(s, si + 1, p, pi, dp)
                || Self::recursive_match(s, si, p, pi + 1, dp);
        }
        if result {
            dp[si][pi] = 1;
        } else {
            dp[si][pi] = 2;
        }
        return result;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::is_match("ho".into(), "ho**".into()), true);
    assert_eq!(Solution::is_match("".into(), "*".into()), true);
    assert_eq!(Solution::is_match("aa".into(), "a".into()), false);
    assert_eq!(Solution::is_match("aa".into(), "*".into()), true);
    assert_eq!(Solution::is_match("cb".into(), "?a".into()), false);
    assert_eq!(Solution::is_match("adceb".into(), "*a*b".into()), true);
    assert_eq!(Solution::is_match("acdcb".into(), "a*c?b".into()), false);
}
