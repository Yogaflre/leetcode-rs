// <数组的度>

// Given a non-empty array of non-negative integers nums, the degree of this array is defined as the maximum frequency of any one of its elements.
// Your task is to find the smallest possible length of a (contiguous) subarray of nums, that has the same degree as nums.

// Example 1:
// Input: nums = [1,2,2,3,1]
// Output: 2
// Explanation:
// The input array has a degree of 2 because both elements 1 and 2 appear twice.
// Of the subarrays that have the same degree:
// [1, 2, 2, 3, 1], [1, 2, 2, 3], [2, 2, 3, 1], [1, 2, 2], [2, 2, 3], [2, 2]
// The shortest length is 2. So return 2.

// Example 2:
// Input: nums = [1,2,2,3,1,4,2]
// Output: 6
// Explanation:
// The degree is 3 because the element 2 is repeated 3 times.
// So [2,2,3,1,4,2] is the shortest subarray, therefore returning 6.

use std::{cmp::Reverse, collections::HashMap};
struct Solution;

impl Solution {
    /*
     * 用hash表记录每个字符的最大值和左右边界(HashMap<num, (size, left_index, right_index)>)
     */
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, (usize, usize, usize)> = HashMap::new();
        nums.into_iter().enumerate().for_each(|(i, n)| {
            if let Some(tuple) = map.get_mut(&n) {
                tuple.0 += 1;
                tuple.2 = i;
            } else {
                map.insert(n, (0, i, i));
            }
        });
        if let Some((_, v)) = map
            .into_iter()
            .max_by_key(|(_, v)| (v.0, Reverse(v.2 - v.1)))
        {
            return (v.2 - v.1 + 1) as i32;
        } else {
            return 0;
        }
    }
}

#[test]
fn run() {
    assert_eq!(Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]), 2);
    assert_eq!(
        Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2]),
        6
    );
}
