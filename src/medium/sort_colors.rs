// <颜色排序>
// Given an array with n objects colored red, white or blue, sort them in-place so that objects of the same color are adjacent, with the colors in the order red, white and blue.
// Here, we will use the integers 0, 1, and 2 to represent the color red, white, and blue respectively.
// Note: You are not suppose to use the library's sort function for this problem.

// Example:
// Input: [2,0,2,1,1,0]
// Output: [0,0,1,1,2,2]

// Follow up:
// A rather straight forward solution is a two-pass algorithm using counting sort.
// First, iterate the array counting number of 0's, 1's, and 2's, then overwrite array with total number of 0's, then 1's and followed by 2's.
// Could you come up with a one-pass algorithm using only constant space?

// 解题思路
// 方法一：交换/排序
//  由于只有3个颜色，可以将0摆在前面2摆在后面1摆在中间
//  从头遍历，如果0就往前交换，如果为2就往后交换
// 方法二：排序算法
struct Solution;
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut left = 0;
        let mut right = nums.len() - 1;
        for i in 0..=right {
            while nums[i] == 2 && i < right {
                nums.swap(i, right);
                right -= 1;
            }
            while nums[i] == 0 && i > left {
                nums.swap(i, left);
                left += 1;
            }
        }
    }
}

#[test]
fn run() {
    let mut orginal = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut orginal);
    assert_eq!(orginal, vec![0, 0, 1, 1, 2, 2]);

    let mut orginal = vec![2, 0, 1];
    Solution::sort_colors(&mut orginal);
    assert_eq!(orginal, vec![0, 1, 2]);
}
