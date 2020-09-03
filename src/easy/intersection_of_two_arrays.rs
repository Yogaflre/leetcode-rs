// <两个数组的交集>

// Given two arrays, write a function to compute their intersection.

// Example 1:
// Input: nums1 = [1,2,2,1], nums2 = [2,2]
// Output: [2]

// Example 2:
// Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
// Output: [9,4]
// Note:

// Each element in the result must be unique.
// The result can be in any order.

use std::collections::HashSet;
struct Solution;
impl Solution {
    /**
     * 利用hashset快速匹配
     */
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums = HashSet::new();
        let mut intersect = HashSet::new();
        for n in nums1 {
            nums.insert(n);
        }
        for n in nums2 {
            if nums.contains(&n) {
                intersect.insert(n);
            }
        }
        return intersect.into_iter().collect();
    }

    /**
     * 排序+遍历
     */
    pub fn intersection2(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.sort();
        nums2.sort();
        let mut result = HashSet::new();
        let mut i = 0;
        let mut j = 0;
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                i += 1;
            } else if nums1[i] > nums2[j] {
                j += 1;
            } else {
                result.insert(nums1[i]);
                i += 1;
                j += 1;
            }
        }
        return result.into_iter().collect();
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
        vec![2]
    );
    assert_eq!(
        Solution::intersection2(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
        vec![4, 9]
    );
}
