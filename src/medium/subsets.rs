// <子集>
// Given a set of distinct integers, nums, return all possible subsets (the power set).
// Note: The solution set must not contain duplicate subsets.

// Example:
// Input: nums = [1,2,3]
// Output:
// [
//   [3],
//   [1],
//   [2],
//   [1,2,3],
//   [1,3],
//   [2,3],
//   [1,2],
//   []
// ]

// 解题思路
// 方法一：动态规划
use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut set: HashSet<Vec<i32>> = HashSet::new();
        Self::subset(nums, &mut set);
        return set.into_iter().collect();
    }

    fn subset(nums: Vec<i32>, set: &mut HashSet<Vec<i32>>) {
        if !set.contains(&nums) {
            set.insert(nums.to_owned());
            for i in 0..nums.len() {
                let mut clone: Vec<i32> = nums.clone();
                clone.remove(i);
                Self::subset(clone, set);
            }
        }
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::subsets(vec![1, 2, 3]),
        vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3]
        ]
    );
}
