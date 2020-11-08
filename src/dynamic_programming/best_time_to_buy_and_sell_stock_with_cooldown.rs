// <最佳买卖股票时机含冷冻期>

// Say you have an array for which the ith element is the price of a given stock on day i.
// Design an algorithm to find the maximum profit. You may complete as many transactions as you like (ie, buy one and sell one share of the stock multiple times) with the following restrictions:
// You may not engage in multiple transactions at the same time (ie, you must sell the stock before you buy again).
// After you sell your stock, you cannot buy stock on next day. (ie, cooldown 1 day)

// Example:
// Input: [1,2,3,0,2]
// Output: 3
// Explanation: transactions = [buy, sell, cooldown, buy, sell]

use std::cmp::max;
struct Solution;
impl Solution {
    /**
     * 寻找递推逻辑
     * 三个动态规划条件：
     *  1.持有一只股票的累计最大收益           dp[i][0]
     *  2.不持有股票处于冷冻期累计最大收益      dp[i][1]
     *  3.不持有股票且不处于冷冻期累计最大收益  dp[i][2]
     */
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        let mut dp = vec![vec![0; prices.len()]; 3];
        // NOTE 持有第一支股票的收益为亏损prices[0]
        dp[0][0] = -prices[0];
        for i in 1..prices.len() {
            // NOTE 当前持股说明: 之前一直持有某一股票 or 之前不持股且不处于冷冻期 + 当天购入股票
            dp[0][i] = max(dp[0][i - 1], dp[2][i - 1] + (-prices[i]));
            // NOTE 当前不持股处于冷冻期说明: 前一天持有一支股票，今天卖出进入冷冻期
            dp[1][i] = dp[0][i - 1] + prices[i];
            // NOTE 当天不持股不处于冷冻期说明: 前一天未持股并且在冷冻期 or 前一天未持股不在冷冻期
            dp[2][i] = max(dp[2][i - 1], dp[1][i - 1]);
        }
        // NOTE 取最后两种情况的最大值，因为第一种情况持有股票一定亏损
        return max(dp[1][prices.len() - 1], dp[2][prices.len() - 1]);
    }

    /**
     * 传统动态规划解法
     * 循环递归遍历所有条件，在递归时进行持久化
     */
    pub fn max_profit2(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        // NOTE key存储条件，value存储当前条件最大profit
        let mut dp: Vec<Vec<i32>> = vec![vec![-1; prices.len()]; prices.len()];
        return Self::recursive(&prices, 0, &mut dp);
    }

    fn recursive(prices: &Vec<i32>, index: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if index >= prices.len() - 1 {
            return 0;
        }
        let mut profit = 0;
        // NOTE 取当前index买入，遍历所有天数的股价，计算第一次买入最大值
        for i in index..prices.len() {
            for j in i..prices.len() {
                let mut tmp = 0;
                if dp[i][j] != -1 {
                    tmp = dp[i][j];
                } else if prices[j] > prices[i] {
                    // NOTE 在当前index买入后，继续递归查看后面还有没有可以买入的股票
                    tmp = prices[j] - prices[i] + Self::recursive(prices, j + 2, dp);
                    dp[i][j] = tmp;
                }
                if tmp > profit {
                    profit = tmp;
                }
            }
        }
        return profit;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::max_profit2(vec![1, 2, 3, 0, 2]), 3);
    assert_eq!(Solution::max_profit2(vec![2, 1, 4]), 3);
    assert_eq!(Solution::max_profit(vec![6, 1, 6, 4, 3, 0, 2]), 7);
}
