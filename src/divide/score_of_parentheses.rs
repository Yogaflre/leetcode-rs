// <括号的分数>

// Given a balanced parentheses string s, return the score of the string.
// The score of a balanced parentheses string is based on the following rule:
//     "()" has score 1.
//     AB has score A + B, where A and B are balanced parentheses strings.
//     (A) has score 2 * A, where A is a balanced parentheses string.

// Example 1:
// Input: s = "()"
// Output: 1

// Example 2:
// Input: s = "(())"
// Output: 2

// Example 3:
// Input: s = "()()"
// Output: 2

// Constraints:
//     2 <= s.length <= 50
//     s consists of only '(' and ')'.
//     s is a balanced parentheses string.

struct Solution;
impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let r = chars.len() - 1;
        return Self::divide(&chars, 0, r);
    }

    /*
     * 1.如果是并括号则循环+=
     * 2.如果是嵌套括则递归
     */
    fn divide(s: &Vec<char>, l: usize, r: usize) -> i32 {
        let mut res = 0;
        let mut li = l;
        let mut lc = 0;
        let mut rc = 0;
        for i in l..=r {
            let c: char = s[i];
            if c == '(' {
                lc += 1;
            } else {
                rc += 1;
                if lc == rc {
                    if lc == 1 {
                        res += 1;
                    } else {
                        res += 2 * Self::divide(s, li + 1, i);
                    }
                    li = i + 1;
                    lc = 0;
                    rc = 0;
                }
            }
        }
        return res;
    }
}
