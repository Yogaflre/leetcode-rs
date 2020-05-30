// <搜索插入位置>

// Given a sorted array and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
// You may assume no duplicates in the array.

// Example 1:
// Input: [1,3,5,6], 5
// Output: 2

// Example 2:
// Input: [1,3,5,6], 2
// Output: 1

// Example 3:
// Input: [1,3,5,6], 7
// Output: 4

// Example 4:
// Input: [1,3,5,6], 0
// Output: 0

struct Solution;
impl Solution {
    /**
     * 二分查找
     */
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0 as i32;
        let mut r = nums.len() as i32 - 1;
        while l <= r {
            let mid = (l + r) / 2;
            if nums[mid as usize] == target {
                return mid;
            } else if nums[mid as usize] < target {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        return l as i32;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
}
