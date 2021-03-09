// <两整数之和>

// Given two integers a and b, return the sum of the two integers without using the operators + and -.

// Example 1:
// Input: a = 1, b = 2
// Output: 3

// Example 2:
// Input: a = 2, b = 3
// Output: 5

// -1000 <= a, b <= 1000

struct Solution;
impl Solution {
    /*
     * a: 010
     * b: 011
     * a ^ b: 001(表示不进位的两数之和)
     * (a & b) << 1: 100(表示进位)
     */
    pub fn get_sum(a: i32, b: i32) -> i32 {
        // NOTE 将未进位之和 +(递归本身) 进位数递归，直到进位数为0
        if b == 0 {
            return a;
        } else {
            return Self::get_sum(a ^ b, (a & b) << 1);
        }
    }
}

#[test]
fn run() {
    assert_eq!(Solution::get_sum(1, 2), 3);
    assert_eq!(Solution::get_sum(2, 3), 5);
}
