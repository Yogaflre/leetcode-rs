// <合并区间>
// Given a collection of intervals, merge all overlapping intervals.

// Example 1:
// Input: [[1,3],[2,6],[8,10],[15,18]]
// Output: [[1,6],[8,10],[15,18]]
// Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].

// Example 2:
// Input: [[1,4],[4,5]]
// Output: [[1,5]]
// Explanation: Intervals [1,4] and [4,5] are considered overlapping.

// 解题思路
// 方法一：
//  优先用第一个元素排序数组，从第一元素开始遍历。由于是排序数组，只需要进行比较下一个元素是否符合题目要求，并且变更下一个区间的起始位置
use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by_key(|x| x[0]);
        let mut result: Vec<Vec<i32>> = vec![];
        let mut i = 0;
        while i < intervals.len() {
            let left = intervals[i][0];
            let mut right = intervals[i][1];
            for j in i..intervals.len() {
                if intervals[j][0] <= right && intervals[j][1] >= right {
                    right = intervals[j][1];
                    i = j;
                }
                if intervals[j][0] >= left && intervals[j][1] <= right {
                    i = j;
                }
            }
            result.push(vec![left, right]);
            i += 1;
        }
        return result;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::merge(vec![vec![1, 4], vec![1, 5]]),
        vec![vec![1, 5]]
    );
    assert_eq!(
        Solution::merge(vec![vec![1, 4], vec![2, 3]]),
        vec![vec![1, 4]]
    );
}
