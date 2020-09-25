// <反转字符串中的元音字母>

// Write a function that takes a string as input and reverse only the vowels of a string.

// Example 1:
// Input: "hello"
// Output: "holle"

// Example 2:
// Input: "leetcode"
// Output: "leotcede"
// Note:
// The vowels does not include the letter "y".

use std::collections::HashSet;
struct Solution;
impl Solution {
    /**
     * 双指针
     * 当两个指针都是元音字母时，则对调位置
     */
    pub fn reverse_vowels(s: String) -> String {
        // NOTE 注意输入字符串为空时直接返回，避免计算r索引位置时出现异常
        if s.len() == 0 {
            return s;
        }
        let mut s: Vec<char> = s.chars().collect();
        let mut l = 0;
        let mut r = s.len() - 1;
        let mut vowels: HashSet<char> = HashSet::new();
        vowels.insert('a');
        vowels.insert('e');
        vowels.insert('i');
        vowels.insert('o');
        vowels.insert('u');
        vowels.insert('A');
        vowels.insert('E');
        vowels.insert('I');
        vowels.insert('O');
        vowels.insert('U');
        while l < r {
            if vowels.contains(&s[l]) && vowels.contains(&s[r]) {
                s.swap(l, r);
                l += 1;
                r -= 1;
            } else if vowels.contains(&s[l]) {
                r -= 1;
            } else {
                l += 1;
            }
        }
        return s.iter().collect::<String>();
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::reverse_vowels("hello".to_string()),
        "holle".to_string()
    );
    assert_eq!(
        Solution::reverse_vowels("leetcode".to_string()),
        "leotcede".to_string()
    );
}
