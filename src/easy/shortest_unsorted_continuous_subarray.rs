// <最短无序连续子数组>

// Given an integer array, you need to find one continuous subarray that if you only sort this subarray in ascending order, then the whole array will be sorted in ascending order, too.
// You need to find the shortest such subarray and output its length.

// Example 1:
// Input: [2, 6, 4, 8, 10, 9, 15]
// Output: 5
// Explanation: You need to sort [6, 4, 8, 10, 9] in ascending order to make the whole array sorted in ascending order.
// Note:
// Then length of the input array is in range [1, 10,000].
// The input array may contain duplicates, so ascending order here means <=.

struct Solution;
impl Solution {
    /**
     * 排序
     * 时间复杂度O(nlogn)   空间复杂度O(n)
     * 对nums数组进行clone并排序，依次比较nums和sorted_nums相同索引位置的元素。当不一致时就是最小范围
     * TODO 优化
     */
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();
        while l < r {
            if nums[l] == sorted_nums[l] {
                l += 1;
            } else if nums[r] == sorted_nums[r] {
                r -= 1;
            } else {
                break;
            }
        }
        if l == r {
            return 0;
        } else {
            return (r - l) as i32 + 1;
        }
    }
}

#[test]
fn run() {
    assert_eq!(Solution::find_unsorted_subarray(vec![1, 2, 3, 4]), 0);
    assert_eq!(
        Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]),
        5
    );
}
