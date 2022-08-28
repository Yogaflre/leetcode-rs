// <最大三角形面积>

// Given an array of points on the X-Y plane points where points[i] = [xi, yi], return the area of the largest triangle that can be formed by any three different points. Answers within 10-5 of the actual answer will be accepted.

// Example 1:
// Input: points = [[0,0],[0,1],[1,0],[0,2],[2,0]]
// Output: 2.00000
// Explanation: The five points are shown in the above figure. The red triangle is the largest.

// Example 2:
// Input: points = [[1,0],[0,0],[0,1]]
// Output: 0.50000

// Constraints:
// 3 <= points.length <= 50
// -50 <= xi, yi <= 50
// All the given points are unique.

struct Solution;

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut area = 0.0;
        for (i, iv) in points.iter().enumerate() {
            for (j, jv) in points[i..].iter().enumerate() {
                for kv in points[j..].iter() {
                    let tmp = Self::triangle_area(iv, jv, kv);
                    if tmp > area {
                        area = tmp;
                    }
                }
            }
        }
        return area;
    }

    fn triangle_area(i: &[i32], j: &[i32], k: &[i32]) -> f64 {
        return 0.5
            * (i[0] * j[1] + j[0] * k[1] + k[0] * i[1] - i[0] * k[1] - j[0] * i[1] - k[0] * j[1])
                .abs() as f64;
    }
}
