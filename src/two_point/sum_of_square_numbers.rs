// <平方数之和>

// Given a non-negative integer c, your task is to decide whether there're two integers a and b such that a2 + b2 = c.

// Example 1:
// Input: 5
// Output: True
// Explanation: 1 * 1 + 2 * 2 = 5

// Example 2:
// Input: 3
// Output: False

struct Solution;
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut l = 0;
        let mut r = (c as f64).sqrt() as i32;
        while l <= r {
            let tmp = l * l + r * r;
            if tmp > c {
                r -= 1;
            } else if tmp < c {
                l += 1;
            } else {
                return true;
            }
        }
        return false;
    }
}

#[test]
fn run() {
    // assert_eq!(Solution::judge_square_sum(3), false);
    // assert_eq!(Solution::judge_square_sum(4), true);
    // assert_eq!(Solution::judge_square_sum(5), true);
    assert_eq!(Solution::judge_square_sum(100000), true);
}
