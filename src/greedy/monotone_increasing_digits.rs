// <单调递增的数字>
// An integer has monotone increasing digits if and only if each pair of adjacent digits x and y satisfy x <= y.
// Given an integer n, return the largest number that is less than or equal to n with monotone increasing digits.

// Example 1:
// Input: n = 10
// Output: 9

// Example 2:
// Input: n = 1234
// Output: 1234

// Example 3:
// Input: n = 332
// Output: 299

// Constraints:
//     0 <= n <= 109

struct Solution;

impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut nums: Vec<char> = n.to_string().chars().collect();
        let mut index = nums.len();
        // 从最后一位开始遍历，如果前一位大于后一位，则对前一位-1
        for i in (1..nums.len()).rev() {
            if nums[i - 1] > nums[i] {
                nums[i - 1] = (nums[i - 1] as u8 - 1) as char;
                index = i; // 保存最后一个修改过的位置
            }
        }
        // 将目标位置后面全部置为9
        for i in index..nums.len() {
            nums[i] = '9';
        }
        return nums.into_iter().collect::<String>().parse::<i32>().unwrap();
    }
}
