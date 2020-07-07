// <每日温度>

// Given a list of daily temperatures T, return a list such that, for each day in the input, tells you how many days you would have to wait until a warmer temperature.
// If there is no future day for which this is possible, put 0 instead.

// For example, given the list of temperatures T = [73, 74, 75, 71, 69, 72, 76, 73], your output should be [1, 1, 4, 2, 1, 1, 0, 0].

// Note: The length of temperatures will be in the range [1, 30000]. Each temperature will be an integer in the range [30, 100].

use std::collections::VecDeque;
struct Solution;
impl Solution {
    /**
     * 栈(窗口)
     * 当栈为空或当前元素小于栈顶元素时候，入栈
     * 否则依次对比当前元素与出栈元素，并记录差值到res
     */
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; t.len()];
        let mut stack: VecDeque<usize> = VecDeque::new();
        for i in 0..t.len() {
            while !stack.is_empty() && t[i] > t[*stack.back().unwrap()] {
                let pre = stack.pop_back().unwrap();
                res[pre] = (i - pre) as i32;
            }
            stack.push_back(i);
        }
        return res;
    }

    /**
     * 暴力解   O(n)
     * 双层循环，寻找比当前值大的温度
     */
    pub fn daily_temperatures2(t: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let last_index = t.len() - 1;
        for i in 0..=last_index {
            let mut tmp = 0;
            for j in (i + 1)..=last_index {
                tmp += 1;
                if t[j] > t[i] {
                    break;
                } else if j == last_index {
                    tmp = 0;
                }
            }
            res.push(tmp);
        }
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
        vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
}
