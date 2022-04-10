// <最大数>
// Given a list of non-negative integers nums, arrange them such that they form the largest number and return it.
// Since the result may be very large, so you need to return a string instead of an integer.

// Example 1:
// Input: nums = [10,2]
// Output: "210"

// Example 2:
// Input: nums = [3,30,34,5,9]
// Output: "9534330"

// Constraints:

//     1 <= nums.length <= 100
//     0 <= nums[i] <= 109

struct Solution;

impl Solution {
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        /*
         * 构造目标数字后排序
         */
        nums.sort_by(|a, b| {
            let a = *a as u64;
            let b = *b as u64;
            let mut digits_a: u64 = 10;
            let mut digits_b: u64 = 10;
            while digits_a <= a {
                digits_a *= 10;
            }
            while digits_b <= b {
                digits_b *= 10;
            }
            return (b * digits_a + a).cmp(&(a * digits_b + b));
        });
        if nums[0] == 0 {
            return "0".to_string();
        } else {
            return nums.into_iter().map(|n| n.to_string()).collect::<String>();
        }
    }
}
