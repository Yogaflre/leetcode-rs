// <字符串编码>

// Given an encoded string, return its decoded string.
// The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being repeated exactly k times. Note that k is guaranteed to be a positive integer.
// You may assume that the input string is always valid; there are no extra white spaces, square brackets are well-formed, etc. Furthermore, you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, k. For example, there will not be input like 3a or 2[4].
// The test cases are generated so that the length of the output will never exceed 105.

// Example 1:
// Input: s = "3[a]2[bc]"
// Output: "aaabcbc"

// Example 2:
// Input: s = "3[a2[c]]"
// Output: "accaccacc"

// Example 3:
// Input: s = "2[abc]3[cd]ef"
// Output: "abcabccdcdcdef"

// Constraints:
// 1 <= s.length <= 30
// s consists of lowercase English letters, digits, and square brackets '[]'.
// s is guaranteed to be a valid input.
// All the integers in s are in the range [1, 300].

struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let cs: Vec<char> = s.chars().collect();
        return Self::decode(1, &cs);
    }

    fn decode(repeat: usize, cs: &[char]) -> String {
        let mut s = String::new();

        let mut i = 0;
        while i < cs.len() {
            if cs[i].is_digit(10) {
                let mut r = 0;
                while let Some(t) = cs[i].to_digit(10) {
                    r = r * 10 + t;
                    i += 1;
                }
                let l = i;
                let mut c = 1;
                while c != 0 {
                    i += 1;
                    if cs[i] == '[' {
                        c += 1;
                    } else if cs[i] == ']' {
                        c -= 1;
                    }
                }
                s.push_str(&Self::decode(r as usize, &cs[l + 1..i]));
            } else {
                s.push(cs[i]);
            }
            i += 1;
        }
        return s.repeat(repeat);
    }
}
