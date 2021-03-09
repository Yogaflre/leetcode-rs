// <4的幂>

// Given an integer n, return true if it is a power of four. Otherwise, return false.
// An integer n is a power of four, if there exists an integer x such that n == 4x.

// Example 1:
// Input: n = 16
// Output: true

// Example 2:
// Input: n = 5
// Output: false

// Example 3:
// Input: n = 1
// Output: true

// Constraints:
// -231 <= n <= 231 - 1

// Follow up: Could you solve it without loops/recursion?

struct Solution;
impl Solution {
    /*
     * 三个判断条件
     * 1.n > 0
     * 2.n是2的幂
     * 3.n-1可以被3整除
     */
    pub fn is_power_of_four(n: i32) -> bool {
        return n > 0 && (n & n - 1) == 0 && (n - 1) % 3 == 0;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::is_power_of_four(1), true);
    assert_eq!(Solution::is_power_of_four(5), false);
    assert_eq!(Solution::is_power_of_four(16), true);
}
