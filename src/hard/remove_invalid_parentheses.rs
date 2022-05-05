// <删除无效的括号>

// Remove the minimum number of invalid parentheses in order to make the input string valid. Return all possible results.
// Note: The input string may contain letters other than the parentheses ( and ).

// Example 1:
// Input: "()())()"
// Output: ["()()()", "(())()"]

// Example 2:
// Input: "(a)())()"
// Output: ["(a)()()", "(a())()"]

// Example 3:
// Input: ")("
// Output: [""]

use std::collections::HashSet;
use std::iter::FromIterator;
struct Solution;
impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let chars: Vec<char> = s.chars().collect();
        let mut res: HashSet<String> = HashSet::new();
        let mut left_err_count = 0;
        let mut right_err_count = 0;
        for c in &chars {
            if *c == '(' {
                left_err_count += 1;
            } else if *c == ')' {
                if left_err_count == 0 {
                    right_err_count += 1;
                } else {
                    left_err_count -= 1;
                }
            }
        }

        Self::dfs(
            &chars,
            0,
            &mut String::new(),
            left_err_count,
            right_err_count,
            0,
            &mut res,
        );
        return Vec::from_iter(res.into_iter());
    }

    fn dfs(
        chars: &Vec<char>,
        index: usize,
        tmp: &mut String,
        lc: i32,
        rc: i32,
        open: i32,
        res: &mut HashSet<String>,
    ) {
        if lc < 0 || rc < 0 || open < 0 {
            return;
        }
        if index == chars.len() {
            if lc == 0 && rc == 0 && open == 0 {
                res.insert(tmp.to_string());
            }
        } else {
            if chars[index] == '(' {
                Self::dfs(&chars, index + 1, &mut tmp.clone(), lc - 1, rc, open, res);
                tmp.push(chars[index]);
                Self::dfs(&chars, index + 1, &mut tmp.clone(), lc, rc, open + 1, res);
            } else if chars[index] == ')' {
                Self::dfs(&chars, index + 1, &mut tmp.clone(), lc, rc - 1, open, res);
                tmp.push(chars[index]);
                Self::dfs(&chars, index + 1, &mut tmp.clone(), lc, rc, open - 1, res);
            } else {
                tmp.push(chars[index]);
                Self::dfs(&chars, index + 1, &mut tmp.clone(), lc, rc, open, res);
            }
        }
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::remove_invalid_parentheses("(a)())()".to_string()),
        vec!["(a())()".to_string(), "(a)()()".to_string()]
    );
    assert_eq!(
        Solution::remove_invalid_parentheses("()())()".to_string()),
        vec!["()()()".to_string(), "(())()".to_string()]
    );
}
