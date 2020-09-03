// <加一>

// Given a non-empty array of digits representing a non-negative integer, increment one to the integer.
// The digits are stored such that the most significant digit is at the head of the list, and each element in the array contains a single digit.
// You may assume the integer does not contain any leading zero, except the number 0 itself.

// Example 1:
// Input: digits = [1,2,3]
// Output: [1,2,4]
// Explanation: The array represents the integer 123.

// Example 2:
// Input: digits = [4,3,2,1]
// Output: [4,3,2,2]
// Explanation: The array represents the integer 4321.

// Example 3:
// Input: digits = [0]
// Output: [1]

// Constraints:
// 1 <= digits.length <= 100
// 0 <= digits[i] <= 9

struct Solution;
impl Solution {
    /**
     * 反向遍历数组，当数值小于9时候直接+1返回，若数值等于9则将当前位置改为0继续遍历
     * 当遍历完数组时，如果数组位置1的元素为0则代表所有都进位，则在首部添加1
     */
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        for i in (0..digits.len()).rev() {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }
            digits[i] = 0;
        }
        if digits[0] == 0 {
            let mut tmp = vec![1];
            tmp.append(&mut digits);
            return tmp;
        }
        return digits;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::plus_one(vec![2, 4, 9, 3, 9]), vec![2, 4, 9, 4, 0]);
    assert_eq!(Solution::plus_one(vec![9, 9]), vec![1, 0, 0]);
    assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    assert_eq!(Solution::plus_one(vec![0]), vec![1]);
}
