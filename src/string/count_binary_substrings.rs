// <计数二进制子串>

// Give a string s, count the number of non-empty (contiguous) substrings that have the same number of 0's and 1's, and all the 0's and all the 1's in these substrings are grouped consecutively.
// Substrings that occur multiple times are counted the number of times they occur.

// Example 1:
// Input: "00110011"
// Output: 6
// Explanation: There are 6 substrings that have equal number of consecutive 1's and 0's: "0011", "01", "1100", "10", "0011", and "01".
// Notice that some of these substrings repeat and are counted the number of times they occur.
// Also, "00110011" is not a valid substring because all the 0's (and 1's) are not grouped together.

// Example 2:
// Input: "10101"
// Output: 4
// Explanation: There are 4 substrings: "10", "01", "10", "01" that have equal number of consecutive 1's and 0's.

// Note:
// s.length will be between 1 and 50,000.
// s will only consist of "0" or "1" characters.

use std::cmp::min;

struct Solution;
impl Solution {
    /*
     * NOTE 找规律：两个字符交错时候，取字符最小的个数
     * "00111" -> "01" "0011" -> min(2, 3) = 2
     */
    pub fn count_binary_substrings(s: String) -> i32 {
        // 定义三个变量：res为最终结果、pre为前一种字符的个数、tmp为当前字符的个数
        let mut res = 0;
        let mut pre = 0;
        let mut tmp = 1;
        let cs: Vec<char> = s.chars().collect();
        // 从1开始遍历，判断当前字符和前一个字符是否想等
        for i in 1..cs.len() {
            if cs[i] == cs[i - 1] {
                tmp += 1;
            } else {
                // NOTE 如果不想等说明出现交错字符，取两种不同字符的最小值
                res += min(pre, tmp);
                pre = tmp;
                tmp = 1;
            }
        }
        // NOTE 循环最后并不计算最后的字符集最小值，返回前再计算一遍
        return res + min(pre, tmp);
    }
}

#[test]
fn run() {
    assert_eq!(Solution::count_binary_substrings("00110011".to_string()), 6);
    assert_eq!(Solution::count_binary_substrings("10101".to_string()), 4);
}
