// <查找排序数组中的第一个和最后一个位值>
// Given an array of integers nums sorted in ascending order, find the starting and ending position of a given target value.
// Your algorithm's runtime complexity must be in the order of O(log n).
// If the target is not found in the array, return [-1, -1].

// Example 1:
// Input: nums = [5,7,7,8,8,10], target = 8
// Output: [3,4]

// Example 2:
// Input: nums = [5,7,7,8,8,10], target = 6
// Output: [-1,-1]

// 解题思路
// 方法一：二分查找
//  当nums[mid] == target时，分别向左向右遍历找到所有等于target的最远index

struct Solution;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if !nums.is_empty() {
            let mut left: i32 = 0;
            let mut right: i32 = nums.len() as i32 - 1;
            if nums.len() == 1 {
                if nums[0] == target {
                    return vec![0, 0];
                } else {
                    return vec![-1, -1];
                }
            }
            while left <= right {
                let mid = (left + right) / 2;
                let mid_val = nums[mid as usize];
                if mid_val == target {
                    left = mid;
                    right = mid;
                    while left > 0 {
                        if nums[left as usize - 1] == target {
                            left = left - 1;
                        } else {
                            break;
                        }
                    }
                    while right < nums.len() as i32 - 1 {
                        if nums[right as usize + 1] == target {
                            right = right + 1
                        } else {
                            break;
                        }
                    }
                    return vec![left as i32, right as i32];
                } else if target < mid_val {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
        }
        return vec![-1, -1];
    }
}
#[test]
fn run() {
    assert_eq!(Solution::search_range(vec![1, 4], 4), vec![1, 1]);
    assert_eq!(Solution::search_range(vec![1], 1), vec![0, 0]);
    assert_eq!(Solution::search_range(vec![2, 2], 1), vec![-1, -1]);
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
        vec![3, 4]
    );
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
        vec![-1, -1]
    );
}
