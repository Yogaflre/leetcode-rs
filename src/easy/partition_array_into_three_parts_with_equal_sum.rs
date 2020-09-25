// <将数组分成和相等的三个部分>

// Given an array of integers, return if and only if we can partition the array into three non-empty parts with equal sums.Atrue
// Formally, we can partition the array if we can find indexes with i+1 < j(A[0] + A[1] + ... + A[i] == A[i+1] + A[i+2] + ... + A[j-1] == A[j] + A[j-1] + ... + A[A.length - 1])

// Example 1:
// Input: A = [0,2,1,-6,6,-7,9,1,2,0,1]
// Output: true
// Explanation: 0 + 2 + 1 = -6 + 6 - 7 + 9 + 1 = 2 + 0 + 1

// Example 2:
// Input: A = [0,2,1,-6,6,7,9,-1,2,0,1]
// Output: false

// Example 3:
// Input: A = [3,3,6,5,-2,2,5,1,-9,4]
// Output: true
// Explanation: 3 + 3 = 6 = 5 - 2 + 2 + 5 + 1 - 9 + 4
// Constraints:
// 3 <= A.length <= 50000
// -10^4 <= A[i] <= 10^4

struct Solution;
impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        let sum: i32 = a.iter().sum();
        if sum % 3 != 0 {
            return false;
        }
        let target = sum / 3;
        let mut tmp = 0;
        let mut count = 0;
        for n in a {
            tmp += n;
            if tmp == target {
                count += 1;
                tmp = 0;
                if count == 3 {
                    return true;
                }
            }
        }
        return false;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1]),
        true
    );
}
