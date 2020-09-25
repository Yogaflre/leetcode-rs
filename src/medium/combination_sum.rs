// <组合总和>
// Given a set of candidate numbers (candidates) (without duplicates) and a target number (target),
// find all unique combinations in candidates where the candidate numbers sums to target.
// The same repeated number may be chosen from candidates unlimited number of times.

// Note:
// All numbers (including target) will be positive integers.
// The solution set must not contain duplicate combinations.

// Example 1:
// Input: candidates = [2,3,6,7], target = 7,
// A solution set is:
// [
//   [7],
//   [2,2,3]
// ]

// Example 2:
// Input: candidates = [2,3,5], target = 8,
// A solution set is:
// [
//   [2,2,2,2],
//   [2,3,3],
//   [3,5]
// ]

// 解题思路
// 方法一：循环+递归
//  使用循环控制candidates减小target不变的情况；使用递归控制candidates不变target减小的情况
//  两者之和则为最终解（类似于找零钱问题）
struct Solution;
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        let mut result: Vec<Vec<i32>> = vec![];
        Solution::recursive(&candidates, target, vec![], &mut result, 0);
        return result;
    }

    fn recursive(
        candidates: &Vec<i32>,
        target: i32,
        tmp: Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        index: usize,
    ) {
        for i in index..candidates.len() {
            if target >= candidates[i] {
                let mut new_tmp = tmp.clone();
                new_tmp.push(candidates[i]);
                if target == candidates[i] {
                    result.push(new_tmp);
                    break;
                } else {
                    Solution::recursive(candidates, target - candidates[i], new_tmp, result, i);
                }
            } else {
                break;
            }
        }
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 6, 7], 7),
        vec![vec![7], vec![2, 2, 3]]
    );

    assert_eq!(
        Solution::combination_sum(vec![2, 3, 5], 8),
        vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
    );
}
