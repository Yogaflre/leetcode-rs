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
use std::collections::{BinaryHeap, HashMap};
struct Solution;
impl Solution {
    /**
     * 桶排序
     * 时间复杂度：O(n)
     */
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count_map: HashMap<i32, usize> = HashMap::new();
        let length = nums.len();
        // 使用hashmap统计个数
        for n in nums {
            if let Some(v) = count_map.get_mut(&n) {
                *v += 1;
            } else {
                count_map.insert(n, 0);
            }
        }
        // NOTE 桶排序，每个索引位置代表出现的频率，值为nums中的具体值（有可能会出现多个值同一频率的情况）
        let mut bucket: Vec<Vec<i32>> = vec![vec![]; length];
        for entry in count_map.iter() {
            let tmp: &mut Vec<i32> = &mut bucket[*entry.1 as usize];
            tmp.push(*entry.0);
        }
        let mut res: Vec<i32> = vec![];
        // 倒序查找(从频率最高的地方开始查找)
        for v in bucket.iter().rev() {
            res.extend(v.iter());
            if res.len() == k as usize {
                return res;
            }
        }
        return res;
    }

    /**
     * 利用堆排序
     * 时间复杂度O：(nlogn)
     */
    pub fn top_k_frequent2(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count_map: HashMap<i32, usize> = HashMap::new();
        for n in nums {
            if let Some(v) = count_map.get_mut(&n) {
                *v += 1;
            } else {
                count_map.insert(n, 0);
            }
        }
        // NOTE 使用堆来保存前k最常出现的元素
        let mut heap: BinaryHeap<(usize, i32)> = BinaryHeap::with_capacity(k as usize);
        for entry in count_map {
            heap.push((entry.1, entry.0));
        }
        let mut res: Vec<i32> = vec![];
        for _ in 0..k {
            res.push(heap.pop().unwrap().1);
        }
        return res;
    }

    /**
     * 计数+排序
     * 历nums，统计个数(n) + 按大小排序(nlogn) + 获取前两个元素
     */
    pub fn top_k_frequent3(nums: Vec<i32>, k: i32) -> Vec<i32> {
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
    assert_eq!(Solution::top_k_frequent2(vec![3, 0, 1, 0], 1), vec![0]);
    assert_eq!(
        Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
        vec![1, 2]
    );
    assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
}
