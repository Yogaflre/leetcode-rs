// <有序数组中的单一元素>

// You are given a sorted array consisting of only integers where every element appears exactly twice, except for one element which appears exactly once. Find this single element that appears only once.
// Follow up: Your solution should run in O(log n) time and O(1) space.

// Example 1:
// Input: nums = [1,1,2,3,3,4,4,8,8]
// Output: 2

// Example 2:
// Input: nums = [3,3,7,7,10,11,11]
// Output: 10

// Constraints:
// 1 <= nums.length <= 10^5
// 0 <= nums[i] <= 10^5

struct Solution;
impl Solution {
    /*
     * 二分查找
     * 时间复杂度：O(logn)
     * 利用奇偶位置的特性，如果没有单个值，那么重复元素的第一个元素一定是偶数位
     */
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut l: i32 = 0;
        let mut r: i32 = nums.len() as i32 - 1;
        // NOTE 因为两两相等比较，所以不需要 <=
        while l < r {
            let mut mid = (l + r) / 2;
            // NOTE 如果不存在单个值，则第一个相等字符一定是偶数位
            if mid % 2 == 1 {
                mid -= 1;
            }
            // NOTE 比较两个值是否相等，如果相等说明在后半部分；不相等则在前半部分
            if nums[mid as usize] == nums[mid as usize + 1] {
                l = mid + 2;
            } else {
                r = mid;
            }
        }
        return nums[l as usize];
    }

    /*
     * 异或运算，因为数组中两两出现，只有单个元素存在
     * 时间复杂度：O(n)
     */
    pub fn single_non_duplicate2(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for n in nums {
            res ^= n;
        }
        return res;
    }
}
