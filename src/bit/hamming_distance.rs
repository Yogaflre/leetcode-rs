// <汉明距离>

// The Hamming distance between two integers is the number of positions at which the corresponding bits are different.
// Given two integers x and y, calculate the Hamming distance.
// Note:
// 0 ≤ x, y < 231.

// Example:
// Input: x = 1, y = 4
// Output: 2
// Explanation:
// 1   (0 0 0 1)
// 4   (0 1 0 0)
// The above arrows point to positions where the corresponding bits are different.

struct Solution;
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut res = 0;
        // 取两个值的异或
        let mut tmp = x ^ y;
        while tmp != 0 {
            // 如果&1=1则表示有1
            if tmp & 1 == 1 {
                res += 1;
            }
            // 然后右移tmp直到为0
            tmp >>= 1;
        }
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::hamming_distance(1, 4), 2);
    assert_eq!(Solution::hamming_distance(93, 73), 2);
}
