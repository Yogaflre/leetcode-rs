// <分割等和子集>

// Given a non-empty array containing only positive integers, find if the array can be partitioned into two subsets such that the sum of elements in both subsets is equal.
// Note:
// Each of the array element will not exceed 100.
// The array size will not exceed 200.

// Example 1:
// Input: [1, 5, 11, 5]
// Output: true
// Explanation: The array can be partitioned as [1, 5, 5] and [11].

// Example 2:
// Input: [1, 2, 3, 5]
// Output: false
// Explanation: The array cannot be partitioned into equal sum subsets.

struct Solution;
impl Solution {
    /**
     * 动态规划
     * 根据初始值，将“备忘录”填充完
     * WHY 需要加深理解，不太熟悉非递归的动态规划
     */
    pub fn can_partition(nums: Vec<i32>) -> bool {
        if nums.is_empty() {
            return true;
        }
        let sum = nums.iter().sum::<i32>();
        if nums.len() == 1 || sum % 2 != 0 {
            return false;
        }
        let mid = (sum / 2) as usize;
        let mut dp = vec![vec![false; mid + 1]; nums.len()];
        if nums[0] <= sum as i32 {
            dp[0][nums[0] as usize] = true;
        }
        for i in 0..nums.len() {
            dp[i][0] = true;
        }
        for i in 1..nums.len() {
            for j in 1..=mid {
                if (j as i32) < nums[i] {
                    dp[i][j] = dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j] || dp[i - 1][j - nums[i] as usize];
                }
            }
        }
        return dp[dp.len() - 1][dp[0].len() - 1];
    }

    /**
     * 回溯法
     * 暴力遍历所有情况
     */
    pub fn can_partition2(nums: Vec<i32>) -> bool {
        if nums.is_empty() {
            return true;
        }
        if nums.len() == 1 {
            return false;
        }
        return Self::backtracking(0, nums);
    }

    fn backtracking(left: i32, nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if left == sum {
            return true;
        } else if left < sum {
            let mut res = false;
            for i in 0..nums.len() {
                let mut tmp = nums.clone();
                tmp.remove(i);
                res |= Self::backtracking(left + nums[i], tmp);
            }
            return res;
        } else {
            return false;
        }
    }
}

#[test]
fn run() {
    assert_eq!(Solution::can_partition(vec![1, 1, 2]), true);
    assert_eq!(Solution::can_partition(vec![23, 13, 11, 7, 6, 5, 5]), true);
    assert_eq!(Solution::can_partition(vec![3, 3, 3, 4, 5]), true);
    assert_eq!(Solution::can_partition(vec![1, 3, 4, 4]), false);
    assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
    assert_eq!(Solution::can_partition(vec![1, 2, 3, 5]), false);
    assert_eq!(Solution::can_partition(vec![1, 2, 10, 11]), true);
}
