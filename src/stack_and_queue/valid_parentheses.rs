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

use std::collections::HashMap;
struct Solution;
impl Solution {
    /*
     * 实现思路与方法二相同（通过‘&&’操作符来对false的flag进行持久化）
     */
    pub fn is_valid(s: String) -> bool {
        let mut deque: Vec<char> = vec![];
        let mut flag: bool = true;
        s.chars().for_each(|x| match x {
            '(' | '[' | '{' => deque.push(x),
            '}' => flag = (deque.pop() == Some('{')) && flag,
            ']' => flag = (deque.pop() == Some('[')) && flag,
            ')' => flag = (deque.pop() == Some(')')) && flag,
            _ => flag = false,
        });
        flag && deque.is_empty()
    }

    /*
     * 1.初始化HashMap，以右括号为key，左括号为value
     * 2.遍历字符，当为左括号时压入栈；当为右括号时取出，并判断是否和对应的括号一致，不一致则返回false
     */
    pub fn is_valid2(s: String) -> bool {
        let mut map: HashMap<char, char> = HashMap::new();
        map.insert('}', '{');
        map.insert(')', '(');
        map.insert(']', '[');
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if map.get(&c).is_none() {
                stack.push(c)
            } else {
                if map[&c] != stack.pop().unwrap_or(' ') {
                    return false;
                }
            }
        }
        return stack.is_empty();
    }
}
