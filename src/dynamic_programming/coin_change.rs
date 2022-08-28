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

struct Solution;
impl Solution {
    /*
     * 保存当前余额需要最小的硬笔数，防止重复遍历
     */
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
        if items[amount as usize] != 0 {
            return items[amount as usize];
        }
        let mut tmp = i32::max_value();
        for c in coins {
            let coin = 1 + Solution::change(coins, amount - c, items);
            if coin > 0 && coin < tmp {
                tmp = coin;
            }
        }
        if tmp == i32::max_value() {
            tmp = -1;
        }
        items[amount as usize] = tmp;
        return tmp;
    }
}
#[test]
fn run() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(Solution::coin_change(vec![2], 3), -1);
}
