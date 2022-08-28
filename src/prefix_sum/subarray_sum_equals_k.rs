// <和位K的子数组>

// Given an array of integers and an integer k, you need to find the total number of continuous subarrays whose sum equals to k.

// Example 1:
// Input:nums = [1,1,1], k = 2
// Output: 2

// Constraints:
// The length of the array is in range [1, 20,000].
// The range of numbers in the array is [-1000, 1000] and the range of the integer k is [-1e7, 1e7].

use std::collections::HashMap;
struct Solution;
impl Solution {
    /*
     * prefix sum array
     * nums:      1 1 1
     * array:   0 1 2 3
     * 根据array可以求得 nums[i..=j].sum() == array[j+1] - array[i] == k
     * 所以我们只需要知道在j前面有多少个array[j+1] - array[i] == k
     * 可以利用HashMap来替代prefix sum array方便的存储个数
     */
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        let mut count = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, 1); // 0是prefix sum array的初始值
        for n in nums {
            sum += n;
            count += map.get(&(sum - k)).unwrap_or(&0);
            *map.entry(sum).or_insert(0) += 1;
        }
        return count;
    }

    /*
     * prefix sum(前缀求和)
     * NOTE 使用prefix sum来替代求和计算，降低O(n)的时间复杂度
     */
    pub fn subarray_sum2(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            let mut prefix_sum = 0;
            for j in i..nums.len() {
                prefix_sum += nums[j];
                if prefix_sum == k {
                    count += 1;
                }
            }
        }
        return count;
    }

    /*
     * 暴力解
     */
    pub fn subarray_sum3(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                if nums[i..=j].iter().sum::<i32>() == k {
                    count += 1;
                }
            }
        }
        return count;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::subarray_sum(
            vec![1, 2, 3, 4, 5, 6, 7, 1, 23, 21, 3, 1, 2, 1, 1, 1, 1, 1, 12, 2, 3, 2, 3, 2, 2],
            6
        ),
        5
    );
    assert_eq!(Solution::subarray_sum(vec![-1, -1, 1], 0), 1);
    assert_eq!(Solution::subarray_sum2(vec![1, 2, 3], 3), 2);
    assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
    assert_eq!(Solution::subarray_sum(vec![1], 1), 1);
}
