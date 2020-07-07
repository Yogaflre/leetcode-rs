// <前k个高频元素>

// Given a non-empty array of integers, return the k most frequent elements.

// Example 1:
// Input: nums = [1,1,1,2,2,3], k = 2
// Output: [1,2]

// Example 2:
// Input: nums = [1], k = 1
// Output: [1]

// Note:
// You may assume k is always valid, 1 ≤ k ≤ number of unique elements.
// Your algorithm's time complexity must be better than O(n log n), where n is the array's size.
// It's guaranteed that the answer is unique, in other words the set of the top k frequent elements is unique.
// You can return the answer in any order.

use std::cmp::Reverse;
use std::collections::HashMap;
struct Solution;
impl Solution {
    /**
     * 计数+排序
     * 历nums，统计个数(n) + 按大小排序(nlogn) + 获取前两个元素
     */
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }
        let mut counts: HashMap<i32, usize> = HashMap::new();
        for n in nums {
            *counts.entry(n).or_insert(0) += 1;
        }
        let mut tuples: Vec<(&i32, &usize)> = counts.iter().collect();
        tuples.sort_by_key(|t| Reverse(t.1));
        return tuples.iter().map(|t| *t.0).take(k as usize).collect();
    }
}

#[test]
fn run() {
    assert_eq!(Solution::top_k_frequent(vec![3, 0, 1, 0], 1), vec![0]);
    assert_eq!(
        Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
        vec![1, 2]
    );
    assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
}
