// <最长的美好子字符串>

// A string s is nice if, for every letter of the alphabet that s contains, it appears both in uppercase and lowercase. For example, "abABB" is nice because 'A' and 'a' appear, and 'B' and 'b' appear. However, "abA" is not because 'b' appears, but 'B' does not.
// Given a string s, return the longest substring of s that is nice. If there are multiple, return the substring of the earliest occurrence. If there are none, return an empty string.

// Example 1:
// Input: s = "YazaAay"
// Output: "aAa"
// Explanation: "aAa" is a nice string because 'A/a' is the only letter of the alphabet in s, and both 'A' and 'a' appear.
// "aAa" is the longest nice substring.

// Example 2:
// Input: s = "Bb"
// Output: "Bb"
// Explanation: "Bb" is a nice string because both 'B' and 'b' appear. The whole string is a substring.

// Example 3:
// Input: s = "c"
// Output: ""
// Explanation: There are no nice substrings.

// Constraints:
//     1 <= s.length <= 100
//     s consists of uppercase and lowercase English letters.

use std::collections::HashSet;

struct Solution;
impl Solution {
    /*
     * 暴力法
     * TODO 优化 -> 滑动窗口、分治
     */
    pub fn longest_nice_substring(s: String) -> String {
        let cs: Vec<char> = s.chars().collect();
        let mut res = String::new();
        for i in 0..s.len() {
            'label: for j in i + 1..s.len() {
                let tmp = &cs[i..=j];
                if tmp.len() <= res.len() {
                    continue;
                }
                let mut set: HashSet<char> = HashSet::new();
                for c in tmp {
                    set.insert(*c);
                }
                for c in tmp {
                    if !set.contains(&c.to_ascii_uppercase())
                        || !set.contains(&c.to_ascii_lowercase())
                    {
                        continue 'label;
                    }
                }
                res = tmp.iter().collect();
            }
        }
        return res;
    }
}
