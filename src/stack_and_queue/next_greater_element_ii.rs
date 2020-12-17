// <下一个更大元素 II>

// Given a circular array (the next element of the last element is the first element of the array), print the Next Greater Number for every element. The Next Greater Number of a number x is the first greater number to its traversing-order next in the array, which means you could search circularly to find its next greater number. If it doesn't exist, output -1 for this number.

// Example 1:
// Input: [1,2,1]
// Output: [2,-1,2]
// Explanation: The first 1's next greater number is 2;
// The number 2 can't find next greater number;
// The second 1's next greater number needs to search circularly, which is also 2.
// Note: The length of given array won't exceed 10000.

struct Solution;
impl Solution {
    /*
     * NOTE 循环数组问题：通常可以将两个同样的数组拼接起来，用来计算循环点
     * [1,2,1] -> [1,2,1,1,2,1]
     * 第一次遍历：[2,-1,-1]
     * 第二次遍历：[2,-1,2]
     */
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let length = nums.len();
        let mut res = vec![-1; length];
        // 使用栈来保存之前较小的元素
        let mut stack: Vec<usize> = vec![];
        for i in 0..nums.len() * 2 {
            // NOTE 遇到比之前大的值，则出栈并记录为当前值
            while !stack.is_empty() && nums[i % length] > nums[stack[stack.len() - 1]] {
                res[stack.pop().unwrap()] = nums[i % length];
            }
            stack.push(i % length);
        }
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::next_greater_elements(vec![1, 2, 1]),
        vec![2, -1, 2]
    );
}
