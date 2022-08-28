// <无重复字符的最长子串>

// Given a string, find the length of the longest substring without repeating characters.

// Example 1:
// Input: "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.

// Example 2:
// Input: "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.

// Example 3:
// Input: "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Note that the answer must be a substring, "pwke" is a subsequence and not a substring.

use std::cmp::max;
use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut res = 0;
        let mut map: HashMap<char, usize> = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        let mut l = 0;
        for (r, c) in chars.iter().enumerate() {
            if map.contains_key(c) {
                l = max(*map.get(c).unwrap() + 1, l);
                map.insert(*c, r);
            } else {
                map.insert(*c, r);
            }
            res = max(res, r - l + 1)
        }
        return res as i32;
    }
}
