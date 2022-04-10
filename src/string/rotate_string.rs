// <旋转字符串>
// Given two strings s and goal, return true if and only if s can become goal after some number of shifts on s.

// A shift on s consists of moving the leftmost character of s to the rightmost position.
//     For example, if s = "abcde", then it will be "bcdea" after one shift.

// Example 1:
// Input: s = "abcde", goal = "cdeab"
// Output: true

// Example 2:
// Input: s = "abcde", goal = "abced"
// Output: false

// Constraints:
//     1 <= s.length, goal.length <= 100
//     s and goal consist of lowercase English letters.

struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        if s == goal {
            return true;
        }

        let cs: Vec<char> = s.chars().collect();
        let gs: Vec<char> = goal.chars().collect();
        // 分割每个字符，对比前/后半部分是否相等
        for i in 1..cs.len() {
            let cs_pre = &cs[..i];
            if let Some(index) = goal.find(&cs_pre.iter().collect::<String>()) {
                if &gs[..index] == &cs[i..] {
                    return true;
                }
            }
        }
        return false;
    }
}
