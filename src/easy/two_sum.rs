// <两数之和>
// 给定一个整数数组，返回两个数字的索引，以便它们加起来成为一个特定的目标。
// 您可以假定每个输入都只有一个解决方案，并且您可能不会两次使用同一元素。

// 例：
// 给定nums = [2，7，11，15]，目标= 9，
// 因为nums [ 0 ] + nums [ 1 ] = 2 + 7 = 9，
// 返回[ 0，1 ]。

// 解题思路
// 两次遍历＋判断

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        for i in 0..len {
            for j in i + 1..len {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}

#[test]
pub fn run() {
    let nums = vec![-1, -2, -3, -4, -5];
    let target = -8;
    assert_eq!(Solution::two_sum(nums, target), vec![2, 4]);
}
