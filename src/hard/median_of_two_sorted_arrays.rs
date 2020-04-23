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

// 解题思路：
//  先把两个数组合并为一个有序数组、再通过该有序数组寻找中位数

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.is_empty() {
            return Self::find_middle(nums2);
        }
        if nums2.is_empty() {
            return Self::find_middle(nums1);
        }
        let mut index1 = 0;
        let mut index2 = 0;
        let mut temp: Vec<i32> = vec![];
        while index1 < nums1.len() && index2 < nums2.len() {
            if nums1[index1] < nums2[index2] {
                temp.push(nums1[index1]);
                index1 += 1;
            } else {
                temp.push(nums2[index2]);
                index2 += 1;
            }
            if index1 == nums1.len() {
                temp.extend_from_slice(&nums2[index2..]);
            }
            if index2 == nums2.len() {
                temp.extend_from_slice(&nums1[index1..]);
            }
        }
        return Self::find_middle(temp);
    }

    fn find_middle(nums: Vec<i32>) -> f64 {
        let length = nums.len();
        let mid = length / 2;
        if length % 2 == 0 {
            (nums[mid] + nums[mid - 1]) as f64 / 2 as f64
        } else {
            nums[mid] as f64
        }
    }
}

#[test]
pub fn run() {
    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.5);
}
