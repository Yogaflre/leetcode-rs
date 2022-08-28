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

struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];
        Self::generate(&mut res, String::new(), n, n);
        return res;
    }

    fn generate(res: &mut Vec<String>, tmp: String, l: i32, r: i32) {
        if l == 0 && r == 0 {
            res.push(tmp.to_owned());
        }
        if l > 0 {
            Self::generate(res, tmp.to_owned() + "(", l - 1, r);
        }
        if l < r {
            Self::generate(res, tmp.to_owned() + ")", l, r - 1);
        }
    }
}
