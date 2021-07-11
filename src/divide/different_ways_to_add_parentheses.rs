// <为运算表达式设计优先级>

// Given a string of numbers and operators, return all possible results from computing all the different possible ways to group numbers and operators. The valid operators are +, - and *.

// Example 1:
// Input: "2-1-1"
// Output: [0, 2]
// Explanation:
// ((2-1)-1) = 0
// (2-(1-1)) = 2

// Example 2:
// Input: "2*3-4*5"
// Output: [-34, -14, -10, -10, 10]
// Explanation:
// (2*(3-(4*5))) = -34
// ((2*3)-(4*5)) = -14
// ((2*(3-4))*5) = -10
// (2*((3-4)*5)) = -10
// (((2*3)-4)*5) = 10

struct Solution;
impl Solution {
    pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        for (i, c) in input.chars().enumerate() {
            if !c.is_numeric() {
                // 递归运算符左右的部分
                let l: Vec<i32> = Self::diff_ways_to_compute(input[..i].to_string());
                let r: Vec<i32> = Self::diff_ways_to_compute(input[i + 1..].to_string());
                // NOTE 计算左右部分所有结果
                for i in l.iter() {
                    for j in r.iter() {
                        match c {
                            '+' => res.push(i + j),
                            '-' => res.push(i - j),
                            '*' => res.push(i * j),
                            _ => (),
                        }
                    }
                }
            }
        }
        // NOTE 如果res结果为空，说明输入的字符串无运算符，为纯数字
        if res.is_empty() {
            return vec![input.parse::<i32>().unwrap()];
        } else {
            return res;
        }
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::diff_ways_to_compute("2-1-1".to_string()),
        vec![2, 0]
    );
    assert_eq!(
        Solution::diff_ways_to_compute("2*3-4*5".to_string()),
        vec![-34, -10, -14, -10, 10]
    );
}
