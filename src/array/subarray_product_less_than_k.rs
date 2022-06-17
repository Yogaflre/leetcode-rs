// <乘积小于 K 的子数组>

// Given an array of integers nums and an integer k, return the number of contiguous subarrays where the product of all the elements in the subarray is strictly less than k.

// Example 1:
// Input: nums = [10,5,2,6], k = 100
// Output: 8
// Explanation: The 8 subarrays that have product less than 100 are:
// [10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6]
// Note that [10, 5, 2] is not included as the product of 100 is not strictly less than k.

// Example 2:
// Input: nums = [1,2,3], k = 0
// Output: 0

// Constraints:

//     1 <= nums.length <= 3 * 104
//     1 <= nums[i] <= 1000
//     0 <= k <= 106

struct Solution;
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut tmp = 1;
        let mut l = 0;
        for r in 0..nums.len() {
            tmp *= nums[r]; // 记录当前所有数字的乘积
            while l <= r && tmp >= k {
                // 从当前乘积中逐渐寻找符合大小的范围
                tmp /= nums[l];
                l += 1;
            }
            count += r - l + 1;
        }
        return count as i32;
    }
}
