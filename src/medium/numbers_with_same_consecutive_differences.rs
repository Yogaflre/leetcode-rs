// <连续差相同的数字>

// Return all non-negative integers of length N such that the absolute difference between every two consecutive digits is K.
// Note that every number in the answer must not have leading zeros except for the number 0 itself. For example, 01 has one leading zero and is invalid, but 0 is valid.
// You may return the answer in any order.

// Example 1:
// Input: N = 3, K = 7
// Output: [181,292,707,818,929]
// Explanation: Note that 070 is not a valid number, because it has leading zeroes.

// Example 2:
// Input: N = 2, K = 1
// Output: [10,12,21,23,32,34,43,45,54,56,65,67,76,78,87,89,98]

// Note:
// 1 <= N <= 9
// 0 <= K <= 9

struct Solution;
impl Solution {
    /**
     * 深度优先遍历。递归求解所有符合条件的数字
     * 注意：在n等于1时候需要添加0数字
     */
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut nums = vec![];
        for i in 1..=9 {
            Self::recursive(n - 1, k, i, i.to_string(), &mut nums);
        }
        if n == 1 {
            nums.push(0);
        }
        return nums;
    }

    fn recursive(n: i32, k: i32, pre: i32, tmp: String, nums: &mut Vec<i32>) {
        if n > 0 {
            for i in 0..=9 {
                if (pre - i).abs() == k {
                    let mut new = tmp.clone();
                    new.push_str(&i.to_string());
                    Self::recursive(n - 1, k, i, new, nums);
                }
            }
        } else {
            nums.push(tmp.parse().unwrap());
        }
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::nums_same_consec_diff(3, 7),
        vec![181, 292, 707, 818, 929]
    );
    assert_eq!(
        Solution::nums_same_consec_diff(2, 1),
        vec![10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98]
    );
}
