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

// 解题思路：使用临时vec来记录不重复子串
//  1.遍历所有字符，内部遍历临时vec（temp）
//  2.当该字符不在temp中，则添加至temp；如果在temp中，则对temp切片（从重复元素的下一个index到尾部）
//  3.将该元素添加至temp末尾，并记录temp的最大长度。最终返回该最大长度
// 优化思路：可以尝试将vec变为list
use std::cmp::max;
use std::collections::HashMap;
struct Solution;
impl Solution {
    /**
     * 滑动窗口
     * 使用HashMap记录每个字符的index，遍历字符数组，每当出现重复字符则修改hashmap中的值
     */
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut len: i32 = 0;
        let s: Vec<char> = s.chars().collect();
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut l = 0;
        for r in 0..s.len() {
            if map.contains_key(&s[r]) {
                l = max(*map.get(&s[r]).unwrap() + 1, l);
                map.entry(s[r]).and_modify(|v| *v = r);
            } else {
                map.insert(s[r], r);
            }
            len = max(len, (r - l + 1) as i32);
        }
        return len;
    }
}

#[test]
pub fn run() {
    assert_eq!(
        Solution::length_of_longest_substring(String::from("abba")),
        2
    );
    assert_eq!(
        Solution::length_of_longest_substring(String::from("abcabcbb")),
        3
    );
    assert_eq!(
        Solution::length_of_longest_substring(String::from("bbbbb")),
        1
    );
    assert_eq!(
        Solution::length_of_longest_substring(String::from("pwwkew")),
        3
    );
    assert_eq!(Solution::length_of_longest_substring(String::from("a")), 1);
    assert_eq!(
        Solution::length_of_longest_substring(String::from("dvdf")),
        3
    );
    assert_eq!(
        Solution::length_of_longest_substring(String::from("dvdf")),
        3
    );
}
