// <填充雨水>
// Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it is able to trap after raining.

// https://leetcode.com/problems/trapping-rain-water/
// The above elevation map is represented by array [0,1,0,2,1,0,1,3,2,1,2,1].
// In this case, 6 units of rain water (blue section) are being trapped.

// Example:
// Input: [0,1,0,2,1,0,1,3,2,1,2,1]
// Output: 6

// 解题思路
// 方法一：双指针
//  从头尾各定义一个指针，left和right，分别向中间遍历。在遍历时分别记录当前最大的index。
//  当left<right时则进入循环，如果left_max < right_max，则将left_max值减去left值（一定会有积水，因为比右侧低），右侧同理
use std::cmp::max;
struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let mut l: usize = 0;
        let mut r: usize = height.len() - 1;
        let mut total: i32 = 0;
        let mut l_max = height[l];
        let mut r_max = height[r];
        while l < r {
            l_max = max(l_max, height[l]);
            r_max = max(r_max, height[r]);
            if l_max < r_max {
                total += l_max - height[l];
                l += 1;
            } else {
                total += r_max - height[r];
                r -= 1;
            }
        }
        return total;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(Solution::trap(vec![2, 1, 3, 1]), 1);
    assert_eq!(Solution::trap(vec![4, 2, 3]), 1);
    assert_eq!(Solution::trap(vec![5, 4, 1, 2]), 1);
}
