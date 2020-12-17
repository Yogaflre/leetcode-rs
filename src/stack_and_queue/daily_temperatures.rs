// <每日温度>

// Given a list of daily temperatures T, return a list such that, for each day in the input, tells you how many days you would have to wait until a warmer temperature.
// If there is no future day for which this is possible, put 0 instead.

// For example, given the list of temperatures T = [73, 74, 75, 71, 69, 72, 76, 73], your output should be [1, 1, 4, 2, 1, 1, 0, 0].

// Note: The length of temperatures will be in the range [1, 30000]. Each temperature will be an integer in the range [30, 100].

struct Solution;
impl Solution {
    /**
     * 栈(窗口)
     */
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; t.len()];
        // NOTE 使用栈来保存每个元素的index位置
        let mut stack: Vec<usize> = vec![];
        for (i, v) in t.iter().enumerate() {
            // NOTE  栈不为空，当前值大于栈顶值时，代表气温比之前的天气高。需要对栈中比当前元素小的温度计算差值
            while !stack.is_empty() && *v > t[stack[stack.len() - 1]] {
                let tmp = stack.pop().unwrap();
                res[tmp] = (i - tmp) as i32;
            }
            // 再每次将最新元素入栈，用作与之后的天气比较
            stack.push(i);
        }

        return res;
    }

    /**
     * 暴力解   O(n^2)
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
