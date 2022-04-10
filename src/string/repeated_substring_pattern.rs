// <重复的字符串>
// Given a string s, check if it can be constructed by taking a substring of it and appending multiple copies of the substring together.

// Example 1:
// Input: s = "abab"
// Output: true
// Explanation: It is the substring "ab" twice.

// Example 2:
// Input: s = "aba"
// Output: false

// Example 3:
// Input: s = "abcabcabcabc"
// Output: true
// Explanation: It is the substring "abc" four times or the substring "abcabc" twice.

// Constraints:
//     1 <= s.length <= 104
//     s consists of lowercase English letters.

struct Solution;
impl Solution {
    /*
     * 将两个s合并，去掉头尾。包含原字符串
     */
    pub fn repeated_substring_pattern(s: String) -> bool {
        let mut tmp = s.clone();
        tmp.push_str(&s);
        return tmp
            .chars()
            .skip(1)
            .take(tmp.len() - 2)
            .collect::<String>()
            .contains(&s);
    }
}
