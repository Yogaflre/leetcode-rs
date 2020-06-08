// <最小窗口子序列>

// Given a string S and a string T, find the minimum window in S which will contain all the characters in T in complexity O(n).

// Example:
// Input: S = "ADOBECODEBANC", T = "ABC"
// Output: "BANC"
// Note:
// If there is no such window in S that covers all characters in T, return the empty string "".
// If there is such window, you are guaranteed that there will always be only one unique minimum window in S.

use std::collections::HashMap;
struct Solution;
impl Solution {
    /**
     * 晃动窗口
     * 首先用map记录每个t中每个字符的个数，并且记录t中的字符总数
     * 取双指针l和r代表窗口的左右边界。从头开始遍历移动r指针
     * 在移动r的过程中，当length==0时，代表l-r的区间中包含了所有元素
     * 此时开始移动l指针，缩小窗口范围，直到length!=0。并记录每次最小的窗口大小
     */
    pub fn min_window(s: String, t: String) -> String {
        let mut length: i32 = t.len() as i32;
        let mut res: String = String::new();
        let s: Vec<char> = s.chars().collect();
        let mut map: HashMap<char, i32> = HashMap::new();

        let mut l = 0;
        for c in t.chars() {
            map.entry(c).or_insert(0);
            map.entry(c).and_modify(|v| *v += 1);
        }
        for r in 0..s.len() {
            if map.contains_key(&s[r]) {
                map.entry(s[r]).and_modify(|v| *v -= 1);
                if *map.get(&s[r]).unwrap() >= 0 {
                    length -= 1;
                }
                while length == 0 {
                    if r - l + 1 < res.len() || res.is_empty() {
                        res = s[l..=r].iter().collect::<String>();
                    }
                    if map.contains_key(&s[l]) {
                        map.entry(s[l]).and_modify(|v| *v += 1);
                        if *map.get(&s[l]).unwrap() > 0 {
                            length += 1;
                        }
                    }
                    l += 1;
                }
            }
        }
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::min_window("acbbaca".to_string(), "aba".to_string()),
        "baca".to_string()
    );
    assert_eq!(
        Solution::min_window("bba".to_string(), "ab".to_string()),
        "ba".to_string()
    );
    assert_eq!(
        Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
        "BANC".to_string()
    );
    assert_eq!(
        Solution::min_window("EABAC".to_string(), "AC".to_string()),
        "AC".to_string()
    );
    assert_eq!(
        Solution::min_window("EABAC".to_string(), "D".to_string()),
        "".to_string()
    );
}
