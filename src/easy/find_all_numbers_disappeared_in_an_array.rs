// <找到所有数组中消失的数字>
// Given an array of integers where 1 ≤ a[i] ≤ n (n = size of array), some elements appear twice and others appear once.
// Find all the elements of [1, n] inclusive that do not appear in this array.
// Could you do it without extra space and in O(n) runtime? You may assume the returned list does not count as extra space.

// Example:
// Input:
// [4,3,2,7,8,2,3,1]
// Output:
// [5,6]

// 解题思路
// 方法一：设置存在标志位(负数)
//  先遍历数组(假设数组从0开始)，依次将数组的值当做索引位置，将目标索引位置的值变为负数(说明该位置的值已存在)
//  遍历完成后，如果某个位置的值为正数，说明该值在数组中不存在
// 方法二：逻辑处理
//  排序、设置标志位，依次比较标志位与元素的大小进行逻辑处理
struct Solution;
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut result: Vec<i32> = vec![];
        for i in 0..nums.len() {
            let tmp = nums[i].abs() as usize - 1;
            if nums[tmp] > 0 {
                nums[tmp] = -nums[tmp];
            }
        }
        for (i, n) in nums.iter().enumerate() {
            if *n > 0 {
                result.push(i as i32 + 1);
            }
        }
        return result;
    }

    pub fn find_disappeared_numbers2(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let mut result = vec![];
        let mut flag = 1;
        let mut i: usize = 0;
        while i < nums.len() {
            if nums[i] == flag {
                i += 1;
                flag += 1;
            } else if nums[i] < flag {
                i += 1;
            } else {
                result.push(flag);
                flag += 1;
            }
        }
        if !nums.is_empty() {
            for n in nums[nums.len() - 1] + 1..=nums.len() as i32 {
                result.push(n);
            }
        }
        return result;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
        vec![5, 6]
    );
    assert_eq!(Solution::find_disappeared_numbers(vec![1, 1]), vec![2]);
    assert_eq!(Solution::find_disappeared_numbers(vec![]), vec![]);
}
