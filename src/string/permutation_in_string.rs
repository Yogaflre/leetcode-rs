// <字符串的排列>

// Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.
// In other words, return true if one of s1's permutations is the substring of s2.

// Example 1:
// Input: s1 = "ab", s2 = "eidbaooo"
// Output: true
// Explanation: s2 contains one permutation of s1 ("ba").

// Example 2:
// Input: s1 = "ab", s2 = "eidboaoo"
// Output: false

// Constraints:
//     1 <= s1.length, s2.length <= 104
//     s1 and s2 consist of lowercase English letters.

struct Solution;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();

        // 统计当前窗口内的字符个数
        let mut c1 = vec![0; 26];
        let mut c2 = vec![0; 26];

        for i in 0..s1.len() {
            c1[(s1[i] as u8 - 'a' as u8) as usize] += 1;
            c2[(s2[i] as u8 - 'a' as u8) as usize] += 1;
        }

        if c1 == c2 {
            return true;
        }
        for i in s1.len()..s2.len() {
            let l = i - s1.len();
            let r = i;
            c2[(s2[l] as u8 - 'a' as u8) as usize] -= 1;
            c2[(s2[r] as u8 - 'a' as u8) as usize] += 1;
            if c1 == c2 {
                return true;
            }
        }
        return false;
    }
}
