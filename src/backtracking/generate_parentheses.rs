// <生成括号>
// Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

// For example, given n = 3, a solution set is:
// [
//   "((()))",
//   "(()())",
//   "(())()",
//   "()(())",
//   "()()()"
// ]

// 解题思路
// 方法一：递归
//  1.当左右括号个数均为0时，将字符串写入vec[]
//  2.当左括号大于0时，拼接左括号，并递归左括号个数-1
//  3.当右括号个数大于左括号个数时，拼接右括号，并递归右括号个数-1
struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let temp: String = String::new();
        Solution::generate(&mut result, temp, n, n);
        return result;
    }

    fn generate(result: &mut Vec<String>, temp: String, l: i32, r: i32) {
        if l == 0 && r == 0 {
            result.push(temp.to_owned());
        }
        if l > 0 {
            Solution::generate(result, temp.to_owned() + "(", l - 1, r);
        }
        if r > l {
            Solution::generate(result, temp.to_owned() + ")", l, r - 1);
        }
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::generate_parenthesis(3),
        vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
    );
}
