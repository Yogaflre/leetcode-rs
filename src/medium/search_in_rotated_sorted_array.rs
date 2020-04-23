// <搜索旋转排序数组>
// Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
// (i.e., [0,1,2,4,5,6,7] might become [4,5,6,7,0,1,2]).
// You are given a target value to search. If found in the array return its index, otherwise return -1.
// You may assume no duplicate exists in the array.
// Your algorithm's runtime complexity must be in the order of O(log n).

// Example 1:
// Input: nums = [4,5,6,7,0,1,2], target = 0
// Output: 4

// Example 2:
// Input: nums = [4,5,6,7,0,1,2], target = 3
// Output: -1

// 解题思路
// 方法一：
//  当nums为空时，直接返回-1
//  当nums长度为1时，判断元素是否为target，是返回0、否返回-1
//  否则，二分查找。
//      当mid值等于target时，返回mid位值；
//      分别判断left<=mid和left>mid的情况，进而判断target的区间
struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if !nums.is_empty() {
            if nums.len() == 1 {
                if nums[0] == target {
                    return 0;
                } else {
                    return -1;
                }
            }
            let mut left: i32 = 0;
            let mut right: i32 = nums.len() as i32 - 1;
            while left <= right {
                let mid = (left + right) / 2;
                let mid_val = nums[mid as usize];
                let left_val = nums[left as usize];
                let right_val = nums[right as usize];
                if target == mid_val {
                    return mid;
                }
                if left_val <= mid_val {
                    if target < mid_val && target >= left_val {
                        right = mid - 1;
                    } else {
                        left = mid + 1;
                    }
                } else {
                    if target > mid_val && target <= right_val {
                        left = mid + 1;
                    } else {
                        right = mid - 1;
                    }
                }
            }
        }
        return -1;
    }
}

#[test]
pub fn run() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 8, 1, 2, 3], 8), 4);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 8, 0, 1, 2], 0), 5);
    assert_eq!(Solution::search(vec![1], 0), -1);
    assert_eq!(Solution::search(vec![1], 1), 0);
    assert_eq!(Solution::search(vec![1, 3], 1), 0);
    assert_eq!(Solution::search(vec![1, 3], 4), -1);
    assert_eq!(Solution::search(vec![3, 5, 1], 3), 0);
    assert_eq!(Solution::search(vec![5, 1, 3], 5), 0);
}
