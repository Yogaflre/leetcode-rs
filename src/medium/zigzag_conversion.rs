// <Z字型转换>
// The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this:
// (you may want to display this pattern in a fixed font for better legibility)

// P   A   H   N
// A P L S I I G
// Y   I   R
// And then read line by line: "PAHNAPLSIIGYIR"
// Write the code that will take a string and make this conversion given a number of rows:
// string convert(string s, int numRows);

// Example 1:
// Input: s = "PAYPALISHIRING", numRows = 3
// Output: "PAHNAPLSIIGYIR"

// Example 2:
// Input: s = "PAYPALISHIRING", numRows = 4
// Output: "PINALSIGYAHRPI"

// Explanation:
// P     I    N
// A   L S  I G
// Y A   H R
// P     I

// 解题思路
// 方法一：
//  当num_rows=1时，直接返回该字符串
//  当num_rows>!时，初始化长度为nums_rows的字符串vec，遍历字符串填充vec，最终将vec合并

struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let nums = num_rows as usize;
        let mut results: Vec<String> = vec![String::new(); nums];

        let mut index = 0;
        let mut flag = true;
        for c in s.chars() {
            results[index].push(c);
            index = if flag { index + 1 } else { index - 1 };
            flag = if flag { index < nums - 1 } else { index < 1 };
        }
        return results.concat();
    }
}

#[test]
pub fn run() {
    assert_eq!(Solution::convert(String::from("AB"), 1), "AB");
    assert_eq!(Solution::convert(String::from("ABCDE"), 3), "AEBDC");
    assert_eq!(
        Solution::convert(String::from("PAYPALISHIRING"), 3),
        "PAHNAPLSIIGYIR"
    );
    assert_eq!(
        Solution::convert(String::from("PAYPALISHIRING"), 4),
        "PINALSIGYAHRPI"
    );
}
