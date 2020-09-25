// <用最少数量的箭引爆气球>

// There are a number of spherical balloons spread in two-dimensional space. For each balloon, provided input is the start and end coordinates of the horizontal diameter. Since it's horizontal, y-coordinates don't matter and hence the x-coordinates of start and end of the diameter suffice. Start is always smaller than end. There will be at most 104 balloons.
// An arrow can be shot up exactly vertically from different points along the x-axis. A balloon with xstart and xend bursts by an arrow shot at x if xstart ≤ x ≤ xend. There is no limit to the number of arrows that can be shot. An arrow once shot keeps travelling up infinitely. The problem is to find the minimum number of arrows that must be shot to burst all balloons.

// Example:
// Input:
// [[10,16], [2,8], [1,6], [7,12]]
// Output:
// 2

// Explanation:
// One way is to shoot one arrow for example at x = 6 (bursting the balloons [2,8] and [1,6]) and another arrow at x = 11 (bursting the other two balloons).

struct Solution;
impl Solution {
    /**
     * 排序 + 贪心
     */
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }
        let mut points = points;
        points.sort();
        // NOTE 初始为1根箭
        let mut count = 1;
        let mut tmp: &Vec<i32> = &points[0];
        for i in 1..points.len() {
            // 新气球左边界大于当前右边界时，说明气球不重叠
            if points[i][0] > tmp[1] {
                tmp = &points[i];
                count += 1;
            } else if points[i][1] < tmp[1] {
                // NOTE 将右边界最小的气球值当作tmp(取最小公共的部分)
                tmp = &points[i];
            }
        }
        return count;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]),
        2
    );
}
