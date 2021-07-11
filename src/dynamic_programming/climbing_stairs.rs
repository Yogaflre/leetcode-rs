// <爬楼梯>
// You are climbing a stair case. It takes n steps to reach to the top.
// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
// Note: Given n will be a positive integer.

// Example 1:
// Input: 2
// Output: 2
// Explanation: There are two ways to climb to the top.
// 1. 1 step + 1 step
// 2. 2 steps

// Example 2:
// Input: 3
// Output: 3
// Explanation: There are three ways to climb to the top.
// 1. 1 step + 1 step + 1 step
// 2. 1 step + 2 steps
// 3. 2 steps + 1 step

struct Solution;
impl Solution {
    /*
     * 动态规划
     * 统计爬1台阶和2个台阶的情况
     */
    pub fn climb_stairs(n: i32) -> i32 {
        let mut items: Vec<i32> = vec![0; n as usize + 1];
        return Solution::climb(n as usize, &mut items);
    }
    fn climb(n: usize, items: &mut Vec<i32>) -> i32 {
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }
        if items[n] != 0 {
            return items[n];
        }
        items[n] = Solution::climb(n - 1, items) + Solution::climb(n - 2, items);
        return items[n as usize];
    }
}

#[test]
fn run() {
    assert_eq!(Solution::climb_stairs(2), 2);
    assert_eq!(Solution::climb_stairs(3), 3);
}
