// <装满水的容器>
// https://leetcode.com/problems/container-with-most-water/

// Given n non-negative integers a1, a2, ..., an , where each represents a point at coordinate (i, ai).
// n vertical lines are drawn such that the two endpoints of line i is at (i, ai) and (i, 0).
// Find two lines, which together with x-axis forms a container, such that the container contains the most water.
// Note: You may not slant the container and n is at least 2.

// Example:
// Input: [1,8,6,2,5,4,8,3,7]
// Output: 49

use std::cmp::{max, min};
struct Solution;
impl Solution {
    /**
     * 双指针
     * 定义首尾指针，并计算面积。移动高度较低的指针，直到两指针重合
     */
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut area = 0;
        while l < r {
            area = max((r - l) as i32 * min(height[l], height[r]), area);
            match height[l] < height[r] {
                true => l += 1,
                false => r -= 1,
            }
        }
        return area;
    }

    /**
     * 暴力搜索
     * 遍历所有面积，返回最大值
     */
    pub fn max_area2(height: Vec<i32>) -> i32 {
        let mut area: i32 = 0;
        let len: usize = height.len();
        for i in 0..len {
            for j in i..len {
                area = max(area, (j - i) as i32 * min(height[i], height[j]));
            }
        }
        return area as i32;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(Solution::max_area(vec![1, 1]), 1);
    assert_eq!(Solution::max_area(vec![2, 1]), 1);
}
