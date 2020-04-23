// <第一个缺失的正整数>
// Given an unsorted integer array, find the smallest missing positive integer.

// Example 1:
// Input: [1,2,0]
// Output: 3

// Example 2:
// Input: [3,4,-1,1]
// Output: 2

// Example 3:
// Input: [7,8,9,11,12]
// Output: 1

// Note:
// Your algorithm should run in O(n) time and uses constant extra space O(1).

// 解题思路
// 方法一：“双循环”
//  遍历数组
//  判断当前元素的值，将该值当做下一个index的位置。内部循环直到下一个index不符合条件
// 方法二：新建空链表
//  定义值为0的新数组（大小与原数组相同）
//  遍历数组，若值大于0或小于索引长度，则将值放入对应新元素索引的位置
//  最后遍历新数组，找到值为0的元素，该位置的索引则为最小缺失元素
struct Solution;
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 1;
        }
        let mut nums = nums;
        for i in 0..nums.len() {
            let mut next_index = nums[i];
            if i as i32 != next_index - 1 {
                nums[i] = 0;
                while next_index > 0
                    && next_index <= nums.len() as i32
                    && nums[next_index as usize - 1] != next_index
                {
                    let tmp = nums[next_index as usize - 1];
                    nums[next_index as usize - 1] = next_index;
                    next_index = tmp;
                }
            }
        }
        for (i, v) in nums.iter().enumerate() {
            if *v == 0 {
                return i as i32 + 1;
            }
        }
        return nums.len() as i32 + 1;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::first_missing_positive(vec![]), 1);
    assert_eq!(Solution::first_missing_positive(vec![1]), 2);
    assert_eq!(Solution::first_missing_positive(vec![1, 1]), 2);
    assert_eq!(Solution::first_missing_positive(vec![2, 1]), 3);
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4]), 5);
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
    assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 1);
    assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
}
