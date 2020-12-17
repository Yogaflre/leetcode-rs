// <同构字符串>

// Given two strings s and t, determine if they are isomorphic.
// Two strings are isomorphic if the characters in s can be replaced to get t.
// All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character but a character may map to itself.

// Example 1:
// Input: s = "egg", t = "add"
// Output: true

// Example 2:
// Input: s = "foo", t = "bar"
// Output: false

// Example 3:
// Input: s = "paper", t = "title"
// Output: true

// Note:
// You may assume both s and t have the same length.

use std::collections::HashMap;
struct Solution;
impl Solution {
    // 记录每个字符的位置
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut s_map: HashMap<char, usize> = HashMap::new();
        let mut t_map: HashMap<char, usize> = HashMap::new();
        for (i, (sc, tc)) in s.chars().zip(t.chars()).enumerate() {
            // NOTE 插入当前位置时，判断该字符返回的上一个位置是否想等
            if s_map.insert(sc, i) != t_map.insert(tc, i) {
                return false;
            }
        }
        return true;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::is_isomorphic(String::from("egg"), String::from("add")),
        true
    );
    assert_eq!(
        Solution::is_isomorphic(String::from("foo"), String::from("bar")),
        false
    );
    assert_eq!(
        Solution::is_isomorphic(String::from("paper"), String::from("title")),
        true
    );
}
