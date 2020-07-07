// <柱状图中最大的矩形>
// https://leetcode.com/problems/largest-rectangle-in-histogram/

// Given n non-negative integers representing the histogram's bar height where the width of each bar is 1, find the area of largest rectangle in the histogram.
// Above is a histogram where width of each bar is 1, given height = [2,1,5,6,2,3].
// The largest rectangle is shown in the shaded area, which has area = 10 unit.

// Example:
// Input: [2,1,5,6,2,3]
// Output: 10

use std::cmp::{max, min};
struct Solution;
impl Solution {
    /**
     * TODO 优化
     */
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        0
    }

    /**
     * BP暴力法
     * 遍历所有面积
     */
    pub fn largest_rectangle_area2(heights: Vec<i32>) -> i32 {
        let mut area = 0;
        for i in 0..heights.len() {
            let mut height = heights[i];
            for j in i..heights.len() {
                height = min(height, heights[j]);
                area = max(area, (j - i + 1) as i32 * height);
            }
        }
        return area;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
}
