// <回文子串>

// Given a string, your task is to count how many palindromic substrings in this string.
// The substrings with different start indexes or end indexes are counted as different substrings even they consist of same characters.

// Example 1:
// Input: "abc"
// Output: 3
// Explanation: Three palindromic strings: "a", "b", "c".

// Example 2:
// Input: "aaa"
// Output: 6
// Explanation: Six palindromic strings: "a", "a", "a", "aa", "aa", "aaa".

// Note: The input string length won't exceed 1000.

struct Solution;
impl Solution {
    /**
     * 中心点扩散
     * 遍历字符数组，以每一个字符为中心进行扩散校验是否为回文
     */
    pub fn count_substrings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let right_index = chars.len() - 1;
        let mut count = 0;
        for i in 0..=right_index {
            count += Self::search(&chars, i, i);
            if i != right_index && chars[i] == chars[i + 1] {
                count += Self::search(&chars, i, i + 1);
            }
        }
        return count;
    }

    fn search(chars: &Vec<char>, mut left: usize, mut right: usize) -> i32 {
        let mut count = 0;
        while right < chars.len() && chars[left] == chars[right] {
            count += 1;
            if left == 0 {
                break;
            }
            left -= 1;
            right += 1;
        }
        return count;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
    assert_eq!(Solution::count_substrings("abc".to_string()), 3);
}
