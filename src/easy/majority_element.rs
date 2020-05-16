// <多数元素>

// Given an array of size n, find the majority element. The majority element is the element that appears more than ⌊ n/2 ⌋ times.
// You may assume that the array is non-empty and the majority element always exist in the array.

// Example 1:
// Input: [3,2,3]
// Output: 3

// Example 2:
// Input: [2,2,1,1,1,2,2]
// Output: 2

use std::collections::HashMap;
struct Solution;
impl Solution {
    /**
     * 多数投票算法
     * 用num和count来存储当前出现最多元素和出现次数。由于多数元素一定过半，所以最后剩下的num一定是大多数元素
     */
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut num = nums[0];
        let mut count = 0;
        for n in nums {
            if count == 0 {
                num = n;
                count = 1;
            } else if n == num {
                count += 1;
            } else {
                count -= 1;
            }
        }
        return num;
    }

    /**
     * HashMap
     * 构造key和存在个数的map，取最大value的key
     */
    pub fn majority_element2(nums: Vec<i32>) -> i32 {
        let mut items: HashMap<i32, usize> = HashMap::new();
        for n in nums {
            let val = items.entry(n).or_insert(0);
            *val += 1;
        }
        let mut max_value = 0;
        let mut result = 0;
        for i in items {
            if i.1 > max_value {
                max_value = i.1;
                result = i.0;
            }
        }
        return result;
    }

    /**
     * 排序
     * 先排序，取n/2位置的元素
     */
    pub fn majority_element3(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        return nums[nums.len() / 2];
    }
}

#[test]
fn run() {
    assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
}
