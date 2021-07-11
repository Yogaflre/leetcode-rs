// <完全平方数>

// Given an integer n, return the least number of perfect square numbers that sum to n.
// A perfect square is an integer that is the square of an integer; in other words, it is the product of some integer with itself. For example, 1, 4, 9, and 16 are perfect squares while 3 and 11 are not.

// Example 1:
// Input: n = 12
// Output: 3
// Explanation: 12 = 4 + 4 + 4.

// Example 2:
// Input: n = 13
// Output: 2
// Explanation: 13 = 4 + 9.

// Constraints:
// 1 <= n <= 10^4

use std::cmp::min;
struct Solution;
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![-1; n as usize + 1];
        return Self::dfs(n as usize, &mut dp);
    }

    fn dfs(n: usize, dp: &mut Vec<i32>) -> i32 {
        if n == 0 || n == 1 {
            return n as i32;
        }
        if dp[n] != -1 {
            return dp[n];
        }
        let mut tmp = Self::sqrt(n);
        let mut res = i32::max_value();
        // NOTE 遍历<=n的所有完全平方数，寻找结果最小值
        while tmp >= 1 {
            res = min(res, Self::dfs(n - tmp, dp));
            // 寻找下一个最大的完全平方数
            tmp = Self::sqrt(tmp - 1);
        }
        dp[n] = res + 1;
        return dp[n];
    }

    /*
     * 找到平方数的最大整数
     */
    fn sqrt(n: usize) -> usize {
        let mut l = 1;
        let mut r = n;
        while l <= r {
            let mid = l + ((r - l) >> 1);
            if mid.pow(2) > n {
                r = mid - 1;
            } else {
                if (mid + 1).pow(2) > n {
                    return mid.pow(2);
                } else {
                    l = mid + 1;
                }
            }
        }
        return 0;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::num_squares(121), 1);
    assert_eq!(Solution::num_squares(7), 4);
    assert_eq!(Solution::num_squares(12), 3);
    assert_eq!(Solution::num_squares(13), 2);
}
