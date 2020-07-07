// <字符串编码>

// Given an encoded string, return its decoded string.
// The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being repeated exactly k times. Note that k is guaranteed to be a positive integer.
// You may assume that the input string is always valid; No extra white spaces, square brackets are well-formed, etc.
// Furthermore, you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, k. For example, there won't be input like 3a or 2[4].

// Example 1:
// Input: s = "3[a]2[bc]"
// Output: "aaabcbc"

// Example 2:
// Input: s = "3[a2[c]]"
// Output: "accaccacc"

// Example 3:
// Input: s = "2[abc]3[cd]ef"
// Output: "abcabccdcdcdef"

// Example 4:
// Input: s = "abc3[cd]xyz"
// Output: "abccdcdcdxyz"

struct Solution;
impl Solution {
    /**
     * 找规律 + 递归
     * 记录各种变量，寻找重复个数，第一个被[]嵌套的字符串
     * 当字符为']'并且为left为0时候，递归[]之中的字符串，并拼接结果
     */
    pub fn decode_string(s: String) -> String {
        if s.len() == 1 || s.is_empty() {
            return s;
        }
        let s: Vec<char> = s.chars().collect();
        let mut num = 0; //重复次数
        let mut res = String::new(); //结果
        let mut inside = String::new(); //递归字符串
        let mut left = 0; //括号个数
        let mut i: usize = 0; //遍历index
        while i < s.len() {
            if s[i] == '[' {
                left += 1;
                if left != 1 {
                    inside.push(s[i]);
                }
            } else if s[i] == ']' {
                left -= 1;
                if left == 0 {
                    let tmp = Self::decode_string(inside);
                    for _ in 0..num {
                        res.push_str(&tmp);
                    }
                    inside = String::new();
                } else {
                    inside.push(s[i]);
                }
            } else {
                if left == 0 {
                    if s[i].is_digit(10) {
                        let l = i;
                        let mut r = l;
                        for j in l + 1..s.len() {
                            if s[j].is_digit(10) {
                                r = j;
                            } else {
                                break;
                            }
                        }
                        num = s[l..=r].iter().collect::<String>().parse::<i32>().unwrap();
                        i = r;
                    } else {
                        res.push(s[i]);
                    }
                } else {
                    inside.push(s[i]);
                }
            }
            i += 1;
        }
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::decode_string("a10[bc]".to_string()),
        "abcbcbcbcbcbcbcbcbcbc".to_string()
    );
    assert_eq!(
        Solution::decode_string("3[a10[bc]]".to_string()),
        "abcbcbcbcbcbcbcbcbcbcabcbcbcbcbcbcbcbcbcbcabcbcbcbcbcbcbcbcbcbc".to_string()
    );
    assert_eq!(
        Solution::decode_string("3[a2[c]]".to_string()),
        "accaccacc".to_string()
    );
}
