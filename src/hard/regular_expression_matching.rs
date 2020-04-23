// <正则表达式匹配>
// Given an input string (s) and a pattern (p), implement regular expression matching with support for '.' and '*'.
// '.' Matches any single character.
// '*' Matches zero or more of the preceding element.
// The matching should cover the entire input string (not partial).

// Note:
// s could be empty and contains only lowercase letters a-z.
// p could be empty and contains only lowercase letters a-z, and characters like . or *.

// Example 1:
// Input:
// s = "aa"
// p = "a"
// Output: false
// Explanation: "a" does not match the entire string "aa".

// Example 2:
// Input:
// s = "aa"
// p = "a*"
// Output: true
// Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".

// Example 3:
// Input:
// s = "ab"
// p = ".*"
// Output: true
// Explanation: ".*" means "zero or more (*) of any character (.)".

// Example 4:
// Input:
// s = "aab"
// p = "c*a*b"
// Output: true
// Explanation: c can be repeated 0 times, a can be repeated 1 time. Therefore, it matches "aab".

// Example 5:
// Input:
// s = "mississippi"
// p = "mis*is*p*."
// Output: false


// 解题思路
// 方法一：回溯+双指针
//  对s和p字符串取索引标志位，用于标识现在访问到第几个字符。分别判断字符是'*'/'.'/‘char‘的情况，递归执行
struct Solution;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        return Solution::rmatch(
            &s.chars().collect::<Vec<char>>()[..],
            0,
            &p.chars().collect::<Vec<char>>()[..],
            0,
        );
    }
    fn rmatch(s: &[char], mut si: usize, p: &[char], mut pi: usize) -> bool {
        if pi == p.len() {
            if si == s.len() {
                return true;
            } else {
                return false;
            }
        }
        if p[pi] == '*' {
            let mut pre = si - 1;
            let tmp = s[pre];
            while pre < s.len() && (s[pre] == tmp || p[pi - 1] == '.') {
                if Solution::rmatch(s, pre + 1, p, pi + 1) {
                    return true;
                }
                pre += 1;
            }
        } else if si < s.len() && (p[pi] == '.' || s[si] == p[pi]) {
            if Solution::rmatch(s, si + 1, p, pi + 1) {
                return true;
            }
        }
        let next = pi + 1;
        if next < p.len() && p[next] == '*' {
            return Solution::rmatch(s, si, p, pi + 2);
        }
        return false;
    }
}

#[test]
pub fn run() {
    assert_eq!(
        Solution::is_match(String::from(""), String::from(".*")),
        true
    );
    assert_eq!(
        Solution::is_match(String::from("bbbba"), String::from(".*a*a")),
        true
    );
    assert_eq!(
        Solution::is_match(String::from("aaba"), String::from("ab*a*c*a")),
        false
    );
    assert_eq!(
        Solution::is_match(String::from("aaa"), String::from("ab*a*c*a")),
        true
    );
    assert_eq!(
        Solution::is_match(String::from("aaa"), String::from("a*a")),
        true
    );
    assert_eq!(
        Solution::is_match(String::from("aa"), String::from("a")),
        false
    );
    assert_eq!(
        Solution::is_match(String::from("aa"), String::from("a*")),
        true
    );
    assert_eq!(
        Solution::is_match(String::from("ab"), String::from(".*")),
        true
    );
    assert_eq!(
        Solution::is_match(String::from("aab"), String::from("c*a*b")),
        true
    );
    assert_eq!(
        Solution::is_match(String::from("mississippi"), String::from("mis*is*p*.")),
        false
    );
    assert_eq!(
        Solution::is_match(String::from("ab"), String::from(".*c")),
        false
    );
}
