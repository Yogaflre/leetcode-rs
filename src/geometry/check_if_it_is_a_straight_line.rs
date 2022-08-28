// <缀点成线>

// You are given an array coordinates, coordinates[i] = [x, y], where [x, y] represents the coordinate of a point. Check if these points make a straight line in the XY plane.

// Example 1:
// Input: coordinates = [[1,2],[2,3],[3,4],[4,5],[5,6],[6,7]]
// Output: true

// Example 2:
// Input: coordinates = [[1,1],[2,2],[3,4],[4,5],[5,6],[7,7]]
// Output: false

// Constraints:
// 2 <= coordinates.length <= 1000
// coordinates[i].length == 2
// -10^4 <= coordinates[i][0], coordinates[i][1] <= 10^4
// coordinates contains no duplicate point.

struct Solution;

impl Solution {
    pub fn check_straight_line(mut coordinates: Vec<Vec<i32>>) -> bool {
        if coordinates.len() < 3 {
            return true;
        }
        coordinates.sort_by_key(|coord| (coord[0], coord[1]));
        for i in 2..coordinates.len() {
            let a = &coordinates[i - 2];
            let b = &coordinates[i - 1];
            let c = &coordinates[i];
            if Self::rate(a, b) != Self::rate(b, c) {
                return false;
            }
        }
        return true;
    }

    fn rate(a: &[i32], b: &[i32]) -> f32 {
        return (b[1] - a[1]) as f32 / (b[0] - a[0]) as f32;
    }
}
