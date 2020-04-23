// <装满水的容器>
// https://leetcode.com/problems/container-with-most-water/

// Given n non-negative integers a1, a2, ..., an , where each represents a point at coordinate (i, ai).
// n vertical lines are drawn such that the two endpoints of line i is at (i, ai) and (i, 0).
// Find two lines, which together with x-axis forms a container, such that the container contains the most water.
// Note: You may not slant the container and n is at least 2.

// Example:
// Input: [1,8,6,2,5,4,8,3,7]
// Output: 49

// 解题思路
// 方法一：（笨办法）
//  遍历所有面积大小的可能，直到找到最大值
// 方法二：
//

use std::cmp;
struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut area: i32 = 0;
        let len: usize = height.len();
        for i in 0..len {
            for j in i..len {
                area = cmp::max(area, (j - i) as i32 * cmp::min(height[i], height[j]));
            }
        }
        return area as i32;
    }

    //TODO 优化
    pub fn max_area2(height: Vec<i32>) -> i32 {
        0
    }
}

#[test]
fn run() {
    assert_eq!(Solution::max_area2(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(Solution::max_area2(vec![1, 1]), 1);
    assert_eq!(Solution::max_area2(vec![2, 1]), 1);
}
