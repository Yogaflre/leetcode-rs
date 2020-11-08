// <Pow(x,n)>

// Implement pow(x, n), which calculates x raised to the power n (xn).

// Example 1:
// Input: 2.00000, 10
// Output: 1024.00000

// Example 2:
// Input: 2.10000, 3
// Output: 9.26100

// Example 3:
// Input: 2.00000, -2
// Output: 0.25000
// Explanation: 2-2 = 1/22 = 1/4 = 0.25

// Note:
// -100.0 < x < 100.0
// n is a 32-bit signed integer, within the range [−231, 231 − 1]

struct Solution;
impl Solution {
    /**
     * 使用折半法快速求幂
     * 参见《SICP》第一章 求幂章节
     */
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut n: i64 = n as i64; // 防止T-MIN转换为T-MAX溢出
        let mut x = x;
        if n == 0 {
            return 1_f64;
        } else if n < 0 {
            n = -n;
            x = 1_f64 / x;
        }
        if n % 2 == 0 {
            return (Solution::my_pow(x, (n / 2) as i32)).powi(2);
        } else {
            return x * Solution::my_pow(x, (n - 1) as i32);
        }
    }
}

#[test]
fn run() {
    assert_eq!(Solution::my_pow(2.00000, 10), 1024.00000);
    assert_eq!(Solution::my_pow(2.10000, 3), 9.261000000000001);
    assert_eq!(Solution::my_pow(2.00000, -2), 0.25000);
    assert_eq!(Solution::my_pow(0.00001, 2147483647), 0.0);
    assert_eq!(Solution::my_pow(2.00000, -2147483647), 0.0);
}
