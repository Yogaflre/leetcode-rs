// <单词拆分>

// Given a non-empty string s and a dictionary wordDict containing a list of non-empty words, determine if s can be segmented into a space-separated sequence of one or more dictionary words.
// Note:
// The same word in the dictionary may be reused multiple times in the segmentation.
// You may assume the dictionary does not contain duplicate words.

// Example 1:
// Input: s = "leetcode", wordDict = ["leet", "code"]
// Output: true
// Explanation: Return true because "leetcode" can be segmented as "leet code".

// Example 2:
// Input: s = "applepenapple", wordDict = ["apple", "pen"]
// Output: true
// Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
//              Note that you are allowed to reuse a dictionary word.

// Example 3:
// Input: s = "catsandog", wordDict = ["cats", "dog", "sand", "and", "cat"]
// Output: false

// 解题思路
// 方法一：DP+双指针
//  左右指针均从0位置开始，循环递增右指针，直到寻找到左右指针之间的字符串在word_dict中
//  接下来进行两种递归：①左右指针均移动至右指针的下一个索引 ②左指针不变，右指针继续后移一位
struct Solution;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        if s.is_empty() || word_dict.is_empty() {
            return false;
        }
        let mut matrix: Vec<Vec<Option<bool>>> = vec![vec![None; s.len()]; s.len()];
        return Self::break_recursive(&s, &word_dict, 0, 0, &mut matrix);
    }

    fn break_recursive(
        s: &String,
        word_dict: &Vec<String>,
        l: usize,
        r: usize,
        matrix: &mut Vec<Vec<Option<bool>>>,
    ) -> bool {
        if r == s.len() - 1 {
            if word_dict.contains(&s[l..=r].to_string()) {
                return true;
            } else {
                return false;
            }
        }
        if matrix[l][r].is_some() {
            return matrix[l][r].unwrap();
        }
        let mut result = false;
        let tmp: String = s[l..=r].to_string();
        if word_dict.contains(&tmp) {
            result |= Self::break_recursive(s, word_dict, r + 1, r + 1, matrix);
        }
        result |= Self::break_recursive(s, word_dict, l, r + 1, matrix);
        matrix[l][r].replace(result);
        return result;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::word_break(
            "aaaaaaa".to_string(),
            vec!["aaaa".to_string(), "aaa".to_string()]
        ),
        true
    );
    assert_eq!(
        Solution::word_break("".to_string(), vec!["".to_string()]),
        false
    );
    assert_eq!(
        Solution::word_break(
            "applepenapple".to_string(),
            vec!["apple".to_string(), "pen".to_string()]
        ),
        true
    );
    assert_eq!(
        Solution::word_break(
            "catsandog".to_string(),
            vec!["cats".to_string(), "dog".to_string(), "and".to_string()]
        ),
        false
    );
    assert_eq!(
        Solution::word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()]
        ),
        true
    );
}
