// <通过删除字母匹配到字典里最长单词>

// Given a string and a string dictionary, find the longest string in the dictionary that can be formed by deleting some characters of the given string. If there are more than one possible results, return the longest word with the smallest lexicographical order. If there is no possible result, return the empty string.

// Example 1:
// Input:
// s = "abpcplea", d = ["ale","apple","monkey","plea"]
// Output:
// "apple"

// Example 2:
// Input:
// s = "abpcplea", d = ["a","b","c"]
// Output:
// "a"
// Note:
// All the strings in the input will only contain lower-case letters.
// The size of the dictionary won't exceed 1,000.
// The length of all the strings in the input won't exceed 1,000.

struct Solution;
impl Solution {
    /**
     * 遍历所有目标字符
     * 每次设定双指针，依次比较目标元素和当前元素是否一致。当目标字符串大小 < 当前结果时，直接查找下一个字符串
     */
    pub fn find_longest_word(s: String, d: Vec<String>) -> String {
        let mut res = "".to_string();
        let s: Vec<char> = s.chars().collect();
        let mut d = d;
        d.sort();
        for target in d {
            if target.len() <= res.len() {
                continue;
            }
            let tmp: Vec<char> = target.chars().collect();
            let mut i = 0;
            let mut j = 0;
            while i < s.len() && j < tmp.len() {
                if s[i] == tmp[j] {
                    j += 1;
                }
                i += 1;
            }
            if j == target.len() {
                res = target;
            }
        }
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::find_longest_word(
            "abpcplea".into(),
            vec!["ale".into(), "apple".into(), "monkey".into(), "plea".into()]
        ),
        "apple".to_string()
    );
}
