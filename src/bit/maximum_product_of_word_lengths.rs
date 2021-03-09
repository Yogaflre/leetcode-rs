// <最大单词长度乘积>

// Given a string array words, find the maximum value of length(word[i]) * length(word[j]) where the two words do not share common letters. You may assume that each word will contain only lower case letters. If no such two words exist, return 0.

// Example 1:
// Input: ["abcw","baz","foo","bar","xtfn","abcdef"]
// Output: 16
// Explanation: The two words can be "abcw", "xtfn".

// Example 2:
// Input: ["a","ab","abc","d","cd","bcd","abcd"]
// Output: 4
// Explanation: The two words can be "ab", "cd".

// Example 3:
// Input: ["a","aa","aaa","aaaa"]
// Output: 0
// Explanation: No such pair of words.

// Constraints:
// 0 <= words.length <= 10^3
// 0 <= words[i].length <= 10^3
// words[i] consists only of lowercase English letters.

use std::cmp::max;
struct Solution;
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        // 使用u32来保存26个字符
        let mut cs: Vec<u32> = vec![0; words.len()];
        for (i, word) in words.iter().enumerate() {
            for c in word.chars() {
                cs[i] |= 1 << (c as u32 - 'a' as u32)
            }
        }
        // 依次比较所有字符是否含有相同字符，并计算最大值
        let mut res = 0;
        for i in 0..words.len() {
            for j in i..words.len() {
                if cs[i] & cs[j] == 0 {
                    res = max(res, (words[i].len() * words[j].len()) as i32);
                }
            }
        }
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::max_product(vec![
            "abcw".to_string(),
            "baz".to_string(),
            "foo".to_string(),
            "bar".to_string(),
            "xtfn".to_string(),
            "abcdef".to_string()
        ]),
        16
    );
    assert_eq!(
        Solution::max_product(vec![
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
            "d".to_string(),
            "cd".to_string(),
            "bcd".to_string(),
            "abcd".to_string()
        ]),
        4
    );
    assert_eq!(
        Solution::max_product(vec![
            "a".to_string(),
            "aa".to_string(),
            "aaa".to_string(),
            "aaaa".to_string()
        ]),
        0
    );
}
