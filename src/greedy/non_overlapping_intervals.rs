// <无重叠区间>

// Given a collection of intervals, find the minimum number of intervals you need to remove to make the rest of the intervals non-overlapping.

// Example 1:
// Input: [[1,2],[2,3],[3,4],[1,3]]
// Output: 1
// Explanation: [1,3] can be removed and the rest of intervals are non-overlapping.

// Example 2:
// Input: [[1,2],[1,2],[1,2]]
// Output: 2
// Explanation: You need to remove two [1,2] to make the rest of intervals non-overlapping.

// Example 3:
// Input: [[1,2],[2,3]]
// Output: 0
// Explanation: You don't need to remove any of the intervals since they're already non-overlapping.
// Note:
// You may assume the interval's end point is always bigger than its start point.
// Intervals like [1,2] and [2,3] have borders "touching" but they don't overlap each other.

struct Solution;
impl Solution {
    /**
     * 排序 + 贪心
     * 对比当前元素最小值和起始元素最大值
     */
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }
        let mut intervals = intervals;
        // 按照左边界
        intervals.sort();
        let mut count = 0;
        let mut tmp: &Vec<i32> = &intervals[0];
        for i in 1..intervals.len() {
            if intervals[i][0] >= tmp[1] {
                // 当最小时大于等于最大值时，说明不相交。并替换tmp为当前tuple
                tmp = &intervals[i];
            } else {
                count += 1;
                // NOTE 如果相交，则将右边界最小的元素设置为tmp
                if intervals[i][1] < tmp[1] {
                    tmp = &intervals[i];
                }
            }
        }
        return count;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
        1
    );
    assert_eq!(
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
        2
    );
    assert_eq!(
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]),
        0
    );
}
