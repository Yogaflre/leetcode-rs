// <交替位二进制数>

// Given a positive integer, check whether it has alternating bits: namely, if two adjacent bits will always have different values.

// Example 1:
// Input: n = 5
// Output: true
// Explanation: The binary representation of 5 is: 101

// Example 2:
// Input: n = 7
// Output: false
// Explanation: The binary representation of 7 is: 111.

// Example 3:
// Input: n = 11
// Output: false
// Explanation: The binary representation of 11 is: 1011.

// Example 4:
// Input: n = 10
// Output: true
// Explanation: The binary representation of 10 is: 1010.

// Example 5:
// Input: n = 3
// Output: false

// Constraints:
// 1 <= n <= 231 - 1

struct Solution;
impl Solution {
    /*
     * 1.对n右移一位取异或：合法数据只会得到111111...
     * 2.针对合法数据，对n和异或后的结果+1取余(1000 & 101)，结果只能为0
     */
    pub fn has_alternating_bits(n: i32) -> bool {
        let tmp = n ^ n >> 1;
        return (n & (tmp + 1)) == 0;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::has_alternating_bits(5), true);
    assert_eq!(Solution::has_alternating_bits(7), false);
    assert_eq!(Solution::has_alternating_bits(11), false);
    assert_eq!(Solution::has_alternating_bits(10), true);
    assert_eq!(Solution::has_alternating_bits(3), false);
    assert_eq!(Solution::has_alternating_bits(4), false);
}
