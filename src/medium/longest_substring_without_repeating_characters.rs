// <最长公共子串>
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
use std::cmp;

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut temp: Vec<char> = vec![];
        let mut size: i32 = 0;
        for i in s.chars() {
            for index in 0..temp.len() {
                if i == temp[index] {
                    temp = temp[index + 1..].to_vec();
                    break;
                }
            }
            temp.push(i);
            size = cmp::max(size, temp.len() as i32);
        }
        size
    }
}

#[test]
pub fn run() {
    let input1: String = String::from("abcabcbb");
    let input2: String = String::from("bbbbb");
    let input3: String = String::from("pwwkew");
    let input4: String = String::from(" ");
    let input5: String = String::from("dvdf");
    assert_eq!(Solution::length_of_longest_substring(input1), 3);
    assert_eq!(Solution::length_of_longest_substring(input2), 1);
    assert_eq!(Solution::length_of_longest_substring(input3), 3);
    assert_eq!(Solution::length_of_longest_substring(input4), 1);
    assert_eq!(Solution::length_of_longest_substring(input5), 3);
}
