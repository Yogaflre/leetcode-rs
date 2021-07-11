// <交换数字>

// Write a function to swap a number in place (that is, without temporary variables).

// Example:
// Input: numbers = [1,2]
// Output: [2,1]

// Note:
//     numbers.length == 2
//     -2147483647 <= numbers[i] <= 2147483647

struct Solution;
impl Solution {
    pub fn swap_numbers(mut numbers: Vec<i32>) -> Vec<i32> {
        numbers[0] += numbers[1];
        numbers[1] = numbers[0] - numbers[1];
        numbers[0] = numbers[0] - numbers[1];
        return numbers;
    }
}

#[test]
fn run() {
    let nums = vec![1, 2];
    let nums_swap = Solution::swap_numbers(nums);
    assert_eq!(vec![2, 1], nums_swap);
}
