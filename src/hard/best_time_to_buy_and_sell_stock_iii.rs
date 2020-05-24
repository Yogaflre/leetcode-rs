// <买卖股票的最佳时机3>

// Say you have an array for which the ith element is the price of a given stock on day i.
// Design an algorithm to find the maximum profit. You may complete at most two transactions.
// Note: You may not engage in multiple transactions at the same time (i.e., you must sell the stock before you buy again).

// Example 1:
// Input: [3,3,5,0,0,3,1,4]
// Output: 6
// Explanation: Buy on day 4 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
//              Then buy on day 7 (price = 1) and sell on day 8 (price = 4), profit = 4-1 = 3.

// Example 2:
// Input: [1,2,3,4,5]
// Output: 4
// Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
//              Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are
//              engaging multiple transactions at the same time. You must sell before buying again.

// Example 3:
// Input: [7,6,4,3,1]
// Output: 0
// Explanation: In this case, no transaction is done, i.e. max profit = 0.

use std::cmp::max;
struct Solution;
impl Solution {
    /**
     * 动态规划
     * 设立l、r两个index，依次遍历所有节点
     * 设定twice标志位标志是否是第二次交易
     * 创建items数组用来保存当前“l”最大的结果集，用于递归校验
     */
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut items = vec![0; prices.len()];
        return Self::profit(&prices, 0, true, &mut items);
    }

    fn profit(prices: &Vec<i32>, index: usize, twice: bool, items: &mut Vec<i32>) -> i32 {
        if index >= prices.len() {
            return 0;
        }
        if items[index as usize] != 0 {
            return items[index as usize];
        }
        let mut result = 0;
        for l in index..prices.len() - 1 {
            for r in l..prices.len() {
                if prices[r] > prices[l] {
                    let mut tmp = prices[r] - prices[l];
                    if twice {
                        tmp += Self::profit(prices, r + 1, false, items);
                    }
                    result = max(result, tmp);
                }
            }
        }
        items[index as usize] = result;
        return result;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}
