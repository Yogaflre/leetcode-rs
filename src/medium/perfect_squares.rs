// <完全平方数>

// Given a positive integer n, find the least number of perfect square numbers (for example, 1, 4, 9, 16, ...) which sum to n.

// Example 1:
// Input: n = 12
// Output: 3
// Explanation: 12 = 4 + 4 + 4.

// Example 2:
// Input: n = 13
// Output: 2
// Explanation: 13 = 4 + 9.

use std::cmp::min;
struct Solution;
impl Solution {
    /**
     * 动态规划
     * TODO 优化
     * 每次缩小n的范围，遍历n/2 -> 1的所有平方数，取最小值
     */
    pub fn num_squares(n: i32) -> i32 {
        let mut items: Vec<i32> = vec![-1; n as usize + 1];
        return Self::recursive(n as usize, &mut items);
    }

    fn recursive(n: usize, items: &mut Vec<i32>) -> i32 {
        if n == 0 || n == 1 {
            return n as i32;
        }
        if items[n] != -1 {
            return items[n];
        }
        let mut res = i32::max_value();
        for i in (1..=(n / 2)).rev() {
            let tmp = i * i;
            if tmp <= n {
                res = min(1 + Self::recursive(n - tmp, items), res);
            }
        }
        items[n] = res;
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::num_squares(7), 4);
    assert_eq!(Solution::num_squares(12), 3);
    assert_eq!(Solution::num_squares(13), 2);
}
