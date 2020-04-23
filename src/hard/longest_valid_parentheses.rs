// <最长有效括号>
// Given a string containing just the characters '(' and ')', find the length of the longest valid (well-formed) parentheses substring.

// Example 1:
// Input: "(()"
// Output: 2
// Explanation: The longest valid parentheses substring is "()"

// Example 2:
// Input: ")()())"
// Output: 4
// Explanation: The longest valid parentheses substring is "()()"

// 解题思路
// 方法一：
//  通过栈的方式，当”(“时入栈，当")"时出栈。在出栈时判断栈是否为空
//      如果栈为空：则代表之前全部为合法括号，把当前索引位置压入栈（记录当前位置用于后续计算）
//      如果栈不为空：则用当前位置索引减去栈中最后一位索引值，得到当前最大合理括号值。和max比较将最大值赋给max

use std::cmp::max;
struct Solution;
impl Solution {
    // WHY 需要加深理解
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack: Vec<i32> = vec![-1];
        let mut sum: i32 = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    sum = max(sum, i as i32 - stack[(stack.len() - 1)]);
                }
            }
        }
        sum
    }
}

#[test]
fn run() {
    assert_eq!(Solution::longest_valid_parentheses("()".to_string()), 2);
    assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
    assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
    assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);
    assert_eq!(
        Solution::longest_valid_parentheses(")()())()()(".to_string()),
        4
    );
    assert_eq!(
        Solution::longest_valid_parentheses("(()(((()".to_string()),
        2
    );
}
