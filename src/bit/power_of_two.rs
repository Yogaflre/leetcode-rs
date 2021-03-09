// <2的幂>

// Given an integer n, return true if it is a power of two. Otherwise, return false.
// An integer n is a power of two, if there exists an integer x such that n == 2x.

// Example 1:
// Input: n = 1
// Output: true
// Explanation: 2^0 = 1

// Example 2:
// Input: n = 16
// Output: true
// Explanation: 2^4 = 16

// Example 3:
// Input: n = 3
// Output: false

// Example 4:
// Input: n = 4
// Output: true

// Example 5:
// Input: n = 5
// Output: false

// Constraints:
// -231 <= n <= 231 - 1

// Follow up: Could you solve it without loops/recursion?

struct Solution;
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        // 负数不等于2的幂，并且2^n和2^n-1没有相同的位
        // 1 -> 0
        // 10 -> 01
        // 100 -> 011
        // 1000 -> 0111
        // ...
        return n > 0 && n & (n - 1) == 0;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::is_power_of_two(1), true);
    assert_eq!(Solution::is_power_of_two(6), false);
    assert_eq!(Solution::is_power_of_two(16), true);
    assert_eq!(Solution::is_power_of_two(3), false);
    assert_eq!(Solution::is_power_of_two(4), true);
    assert_eq!(Solution::is_power_of_two(5), false);
}
