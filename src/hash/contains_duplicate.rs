// <存在重复元素>

// Given an array of integers, find if the array contains any duplicates.
// Your function should return true if any value appears at least twice in the array, and it should return false if every element is distinct.

// Example 1:
// Input: [1,2,3,1]
// Output: true

// Example 2:
// Input: [1,2,3,4]
// Output: false

// Example 3:
// Input: [1,1,1,3,3,4,3,2,4,2]
// Output: true

use std::collections::HashSet;
struct Solution;
impl Solution {
    /*
     * 时间复杂度：O(1)     空间复杂度：O(n)
     * 用hashset来快速定位重复元素
     */
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for n in nums {
            if set.contains(&n) {
                return true;
            } else {
                set.insert(n);
            }
        }
        return false;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
    assert_eq!(
        Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
        true
    );
}
