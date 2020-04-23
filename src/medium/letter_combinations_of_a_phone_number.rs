// <电话号码的字符组合>
// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.
// A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.

// Example:
// Input: "23"
// Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].

// Note:
// Although the above answer is in lexicographical order, your answer could be in any order you want.

// 解题思路
// 方法一：循环
//  首先根据手机按键初始化字母HashMap。依次遍历号码，对每一组号码进行combine操作（combine用来聚合两组vec），将最终的combine结果返回

use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut map: HashMap<char, Vec<String>> = HashMap::new();
        map.insert(
            '2',
            vec![String::from("a"), String::from("b"), String::from("c")],
        );
        map.insert(
            '3',
            vec![String::from("d"), String::from("e"), String::from("f")],
        );
        map.insert(
            '4',
            vec![String::from("g"), String::from("h"), String::from("i")],
        );
        map.insert(
            '5',
            vec![String::from("j"), String::from("k"), String::from("l")],
        );
        map.insert(
            '6',
            vec![String::from("m"), String::from("n"), String::from("o")],
        );
        map.insert(
            '7',
            vec![
                String::from("p"),
                String::from("q"),
                String::from("r"),
                String::from("s"),
            ],
        );
        map.insert(
            '8',
            vec![String::from("t"), String::from("u"), String::from("v")],
        );
        map.insert(
            '9',
            vec![
                String::from("w"),
                String::from("x"),
                String::from("y"),
                String::from("z"),
            ],
        );

        let d: Vec<char> = digits.chars().collect();
        let mut temp: Vec<String> = Vec::default();
        for i in d {
            let value: Vec<String> = map.get(&i).unwrap().to_owned();
            if temp.is_empty() {
                temp = value;
            } else {
                temp = Solution::combine(temp, value);
            }
        }
        return temp;
    }

    //构造两组字符数组所有组合（eg：["a","b","c"],["d","e","f"] -> "ad","ae","af","bd","be","bf","cd","ce","cf"）
    fn combine(base: Vec<String>, next: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for i in base {
            for j in &next {
                let mut temp: String = i.clone();
                temp.push_str(j);
                result.push(temp);
            }
        }
        return result;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::letter_combinations("23".to_string()),
        vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
}
