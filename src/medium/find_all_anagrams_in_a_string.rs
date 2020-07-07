// <找到字符串中所有字母异位词>

// Given a string s and a non-empty string p, find all the start indices of p's anagrams in s.
// Strings consists of lowercase English letters only and the length of both strings s and p will not be larger than 20,100.
// The order of output does not matter.

// Example 1:
// Input:
// s: "cbaebabacd" p: "abc"
// Output:
// [0, 6]
// Explanation:
// The substring with start index = 0 is "cba", which is an anagram of "abc".
// The substring with start index = 6 is "bac", which is an anagram of "abc".

// Example 2:
// Input:
// s: "abab" p: "ab"
// Output:
// [0, 1, 2]
// Explanation:
// The substring with start index = 0 is "ab", which is an anagram of "ab".
// The substring with start index = 1 is "ba", which is an anagram of "ab".
// The substring with start index = 2 is "ab", which is an anagram of "ab".

use std::collections::HashMap;
struct Solution;
impl Solution {
    /**
     * WHY 不理解了，什么操作
     */
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut items: Vec<i32> = vec![0; 26];
        p.chars()
            .for_each(|c| items[(c as usize) - ('a' as usize)] += 1);
        let s: Vec<char> = s.chars().collect();
        let check = |items: &Vec<i32>| {
            for j in items {
                if *j != 0 {
                    return false;
                }
            }
            return true;
        };
        for i in 0..s.len() {
            items[(s[i] as usize) - ('a' as usize)] -= 1;
            if i >= p.len() {
                items[(s[i - p.len()] as usize) - ('a' as usize)] += 1;
            }
            if i >= p.len() - 1 && check(&items) {
                res.push((i + 1 - p.len()) as i32);
            }
        }
        return res;
    }

    /**
     * 滑动窗口
     * 移动指针来计算符合题意的p区间
     */
    pub fn find_anagrams2(s: String, p: String) -> Vec<i32> {
        let s: Vec<char> = s.chars().collect();
        let mut indexs: Vec<i32> = vec![];
        let mut l = 0;
        let mut r = 0;
        let map: HashMap<char, usize> = p.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        let mut tmp = map.clone();
        while r < s.len() {
            if let Some(val) = tmp.get_mut(&s[r]) {
                if *val > 1 {
                    *val -= 1;
                } else {
                    tmp.remove(&s[r]);
                }
                r += 1;
                if tmp.is_empty() {
                    indexs.push(l as i32);
                    tmp.insert(s[l], 1);
                    l += 1;
                }
            } else {
                l += p.len() - map.values().sum::<usize>() + 1;
                r = l;
                tmp = map.clone();
            }
        }
        return indexs;
    }

    /**
     * 暴力解O(n^2)
     * 对s的每一个位置都当做起点，遍历后面的元素是否在p中(p使用hashmap做快速索引)
     */
    pub fn find_anagrams3(s: String, p: String) -> Vec<i32> {
        let s: Vec<char> = s.chars().collect();
        let mut indexs = vec![];
        let map: HashMap<char, usize> = p.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        for i in 0..s.len() {
            let mut tmp: HashMap<char, usize> = map.clone();
            for j in i..s.len() {
                if let Some(val) = tmp.get_mut(&s[j]) {
                    if *val > 1 {
                        *val -= 1;
                    } else {
                        tmp.remove(&s[j]);
                    }
                } else {
                    break;
                }
            }
            if tmp.is_empty() {
                indexs.push(i as i32);
            }
        }
        return indexs;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::find_anagrams("baa".to_string(), "aa".to_string()),
        vec![1]
    );
    assert_eq!(
        Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
        vec![0, 6]
    );
    assert_eq!(
        Solution::find_anagrams("cbaebabcacd".to_string(), "abc".to_string()),
        vec![0, 5, 6]
    );
    assert_eq!(
        Solution::find_anagrams("abab".to_string(), "ab".to_string()),
        vec![0, 1, 2]
    );
}
