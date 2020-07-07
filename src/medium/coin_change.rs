// <零钱兑换>
// You are given coins of different denominations and a total amount of money amount.
// Write a function to compute the fewest number of coins that you need to make up that amount.
// If that amount of money cannot be made up by any combination of the coins, return -1.

// Example 1:
// Input: coins = [1, 2, 5], amount = 11
// Output: 3
// Explanation: 11 = 5 + 5 + 1

// Example 2:
// Input: coins = [2], amount = 3
// Output: -1
// Note: You may assume that you have an infinite number of each kind of coin.

// 解题思路
// 方法一：动态规划
//  构建长度为amount的数组，用于存储各个余额的最小钱数。遍历coins递归求解该余额的最小钱数
struct Solution;
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut items: Vec<i32> = vec![0; amount as usize + 1];
        return Solution::change(&coins, amount, &mut items);
    }
    fn change(coins: &Vec<i32>, amount: i32, items: &mut Vec<i32>) -> i32 {
        if amount < 0 {
            return -1;
        }
        if amount == 0 {
            return 0;
        }
        let index: usize = amount as usize;
        if items[index] != 0 {
            return items[index];
        }
        let mut tmp = i32::max_value();
        for i in 0..coins.len() {
            let num = Solution::change(coins, amount - coins[i], items);
            if num >= 0 && num < tmp {
                tmp = num + 1;
            }
        }
        if tmp == i32::max_value() {
            items[index] = -1;
        } else {
            items[index] = tmp;
        }
        return items[index];
    }
}
#[test]
fn run() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(Solution::coin_change(vec![2], 3), -1);
}
