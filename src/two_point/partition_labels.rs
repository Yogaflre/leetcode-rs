// <分区标签>

// A string S of lowercase English letters is given. We want to partition this string into as many parts as possible so that each letter appears in at most one part, and return a list of integers representing the size of these parts.

// Example 1:
// Input: S = "ababcbacadefegdehijhklij"
// Output: [9,7,8]
// Explanation:
// The partition is "ababcbaca", "defegde", "hijhklij".
// This is a partition so that each letter appears in at most one part.
// A partition like "ababcbacadefegde", "hijhklij" is incorrect, because it splits S into less parts.

// Note:
// S will have length in range [1, 500].
// S will consist of lowercase English letters ('a' to 'z') only.

use std::collections::HashMap;

struct Solution;
impl Solution {
    /**
     * 第一次遍历：记录每个字符的双指针
     * 第二次遍历：找到最大区间
     */
    pub fn partition_labels(s: String) -> Vec<i32> {
        if s.is_empty() {
            return vec![];
        }
        let s: Vec<char> = s.chars().collect();
        // NOTE 记录每个字符的左右区间
        let mut map: HashMap<char, (usize, usize)> = HashMap::new();
        for i in 0..s.len() {
            match map.get_mut(&s[i]) {
                Some((_, right)) => {
                    *right = i;
                }
                None => {
                    map.insert(s[i], (i, i));
                }
            }
        }
        let mut res: Vec<i32> = vec![];
        // 依次遍历寻找最大区间
        let first_range: &(usize, usize) = map.get(&s[0]).unwrap();
        let mut l: usize = first_range.0;
        let mut r: usize = first_range.1;
        for c in s[1..].iter() {
            let range: &(usize, usize) = map.get(c).unwrap();
            // NOTE 当前字符左边界 > r时候，说明之前都没有出现过该字符，需要创建新的序列
            if range.0 > r {
                res.push((r - l + 1) as i32);
                l = range.0;
                r = range.1;
            } else if range.1 > r {
                r = range.1;
            }
        }
        // NOTE 防止最后一段未记录
        res.push((r - l + 1) as i32);
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::partition_labels("ababcbacadefegdehijhklij".to_string()),
        vec![9, 7, 8]
    );
    assert_eq!(
        Solution::partition_labels("eaaaabaaec".to_string()),
        vec![9, 1]
    );
    assert_eq!(Solution::partition_labels("ac".to_string()), vec![1, 1]);
}
