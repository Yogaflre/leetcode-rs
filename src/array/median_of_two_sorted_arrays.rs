// <寻找两个有序数组的中位数>
// There are two sorted arrays nums1 and nums2 of size m and n respectively.
// Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).
// You may assume nums1 and nums2 cannot be both empty.

// Example 1:
// nums1 = [1, 3]
// nums2 = [2]
// The median is 2.0

// Example 2:
// nums1 = [1, 2]
// nums2 = [3, 4]
// The median is (2 + 3)/2 = 2.5

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged_nums = vec![-1; nums1.len() + nums2.len()]; // 合并两个有序数组
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        while i < nums1.len() || j < nums2.len() {
            if i < nums1.len() && j < nums2.len() {
                if nums1[i] < nums2[j] {
                    merged_nums[k] = nums1[i];
                    i += 1;
                } else {
                    merged_nums[k] = nums2[j];
                    j += 1;
                }
            } else if i < nums1.len() {
                merged_nums[k] = nums1[i];
                i += 1;
            } else {
                merged_nums[k] = nums2[j];
                j += 1;
            };
            k += 1;
        }
        let mid_index = merged_nums.len() / 2;
        return if merged_nums.len() % 2 == 0 {
            (merged_nums[mid_index] + merged_nums[mid_index - 1]) as f64 / 2 as f64
        } else {
            merged_nums[mid_index] as f64
        };
    }
}

#[test]
pub fn run() {
    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.5);
}
