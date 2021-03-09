// <数字的补数>

// Given a positive integer num, output its complement number. The complement strategy is to flip the bits of its binary representation.

// Example 1:
// Input: num = 5
// Output: 2
// Explanation: The binary representation of 5 is 101 (no leading zero bits), and its complement is 010. So you need to output 2.

// Example 2:
// Input: num = 1
// Output: 0
// Explanation: The binary representation of 1 is 1 (no leading zero bits), and its complement is 0. So you need to output 0.

// Constraints:
// The given integer num is guaranteed to fit within the range of a 32-bit signed integer.
// num >= 1
// You could assume no leading zero bit in the integer’s binary representation.
// This question is the same as 1009: https://leetcode.com/problems/complement-of-base-10-integer/

struct Solution;
impl Solution {
    /*
     * 因为"n + n的补数" = "无符号数的最大值"(101 + 010 = 111)
     * 所以需要找到同当前num位数相同的最大值(无符号)，再用最大值减去num得到补数
     */
    pub fn find_complement(num: i32) -> i32 {
        let mut tmp = 0;
        // 因为num >= 1，所以当tmp < num时可以确定tmp的位数小于num(全是1的tmp永远是当前位无符号最大值)
        while tmp < num {
            tmp = (tmp << 1) | 1;
        }
        let res = tmp - num;
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::find_complement(5), 2);
    assert_eq!(Solution::find_complement(1), 0);
    assert_eq!(Solution::find_complement(2), 1);
}
