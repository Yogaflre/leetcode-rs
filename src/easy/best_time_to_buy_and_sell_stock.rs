// <买卖股票的最佳时机>

// Say you have an array for which the ith element is the price of a given stock on day i.
// If you were only permitted to complete at most one transaction (i.e., buy one and sell one share of the stock), design an algorithm to find the maximum profit.
// Note that you cannot sell a stock before you buy one.

// Example 1:
// Input: [7,1,5,3,6,4]
// Output: 5
// Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
//              Not 7-1 = 6, as selling price needs to be larger than buying price.

// Example 2:
// Input: [7,6,4,3,1]
// Output: 0
// Explanation: In this case, no transaction is done, i.e. max profit = 0.

// 解题思路
// 方法一：
// 方法二：暴力所有情况
use std::cmp::{max, min};
struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut min_price = i32::max_value();
        for price in prices {
            min_price = min(price, min_price);
            profit = max(profit, price - min_price);
        }
        return profit;
    }

    pub fn max_profit2(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..prices.len() {
            let mut profit = 0;
            for j in i + 1..prices.len() {
                if prices[j] > prices[i] {
                    profit = max(profit, prices[j] - prices[i]);
                }
            }
            result = max(result, profit);
        }
        return result;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::max_profit(vec![3, 2, 6, 5, 0, 3]), 4);
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}
