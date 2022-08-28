// <最长连续序列>

// Given an unsorted array of integers, find the length of the longest consecutive elements sequence.
// Your algorithm should run in O(n) complexity.

// Example:
// Input: [100, 4, 200, 1, 3, 2]
// Output: 4
// Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.

use std::cmp::max;
use std::collections::HashSet;
struct Solution;
impl Solution {
    /*
     * <Hash表>
     */
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // 使用hash表存储，方便查找确定连续元素
        let set: HashSet<i32> = nums.into_iter().collect::<HashSet<i32>>();
        let mut res = 0;
        for start in &set {
            // NOTE 前一个数字不在set中，说明是起始数字。从起始数字开始遍历找到最多的连续数字
            if !set.contains(&(start - 1)) {
                let mut end = *start + 1;
                while set.contains(&end) {
                    end += 1;
                }
                res = max(res, end - start);
            }
        }
        return res;
    }

    /*
     * 排序 + 遍历
     * 时间复杂度：O(nlogn)(不符合题意)
     */
    pub fn longest_consecutive2(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut nums = nums;
        nums.sort();
        let mut result = 1;
        let mut res = 1;
        let mut tmp = nums[0];
        for n in nums {
            if tmp + 1 == n {
                res += 1;
                result = max(res, result);
            } else if tmp != n {
                res = 1;
            }
            tmp = n;
        }
        return result;
    }
}
