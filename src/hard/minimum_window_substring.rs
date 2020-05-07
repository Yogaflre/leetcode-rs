// <最小窗口子序列>
// Given a string S and a string T, find the minimum window in S which will contain all the characters in T in complexity O(n).

// Example:
// Input: S = "ADOBECODEBANC", T = "ABC"
// Output: "BANC"
// Note:
// If there is no such window in S that covers all characters in T, return the empty string "".
// If there is such window, you are guaranteed that there will always be only one unique minimum window in S.

// 解题思路
// 方法一：双指针
//
struct Solution;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut left = 0;
        let mut right = 0;
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        for i in 0..s.len() {
            
        }
        String::default()
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
        "BANC".to_string()
    );
}
