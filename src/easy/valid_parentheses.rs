// <有效的括号>
// Given a string containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
// An input string is valid if:
// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Note that an empty string is also considered valid.

// Example 1:
// Input: "()"
// Output: true

// Example 2:
// Input: "()[]{}"
// Output: true

// Example 3:
// Input: "(]"
// Output: false

// Example 4:
// Input: "([)]"
// Output: false

// Example 5:
// Input: "{[]}"
// Output: true

//解题思路
//方法一：match匹配模式
//  实现思路与方法二相同（通过‘&&’操作符来对false的flag进行持久化）
//方法二：
//  1.初始化HashMap，以右括号为key，左括号为value
//  2.遍历字符，当为左括号时压入栈；当为右括号时取出，并判断是否和对应的括号一致，不一致则返回false

use std::collections::{HashMap, VecDeque};
struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut deque: VecDeque<char> = VecDeque::new();
        let mut flag: bool = true;
        s.chars().for_each(|x| match x {
            '(' | '[' | '{' => deque.push_front(x),
            '}' => flag = (deque.pop_front() == Some('{')) && flag,
            ']' => flag = (deque.pop_front() == Some('[')) && flag,
            ')' => flag = (deque.pop_front() == Some(')')) && flag,
            _ => flag = false,
        });
        flag && deque.is_empty()
    }

    pub fn is_valid2(s: String) -> bool {
        let mut deque: VecDeque<char> = VecDeque::new();
        let mut map: HashMap<char, char> = HashMap::new();
        map.insert('}', '{');
        map.insert(']', '[');
        map.insert(')', '(');
        for i in s.chars() {
            if map.get(&i).is_none() {
                deque.push_front(i);
            } else {
                if map[&i] != deque.pop_front().unwrap_or(' ') {
                    return false;
                }
            }
        }
        return deque.is_empty();
    }
}

#[test]
fn run() {
    assert_eq!(Solution::is_valid(String::from("{[]}")), true);
    assert_eq!(Solution::is_valid(String::from("([)]")), false);
    assert_eq!(Solution::is_valid(String::from("[")), false);
    assert_eq!(Solution::is_valid(String::from("]")), false);
}
