// <寻找旋转排序数组中的最小值>

// Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
// (i.e.,  [0,1,2,4,5,6,7] might become  [4,5,6,7,0,1,2]).
// Find the minimum element.
// You may assume no duplicate exists in the array.

// Example 1:
// Input: [3,4,5,1,2]
// Output: 1

// Example 2:
// Input: [4,5,6,7,0,1,2]
// Output: 0

struct Solution;
impl Solution {
    /**
     * 经典二分查找
     */
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        // 最终会收敛到一个元素，这个元素就是目标元素
        while l < r {
            let mid = l + (r - l) / 2;
            if nums[mid] > nums[r] {
                // NOTE 当中点值大于右边界时，说明右侧有反转，最小值在右侧
                l = mid + 1;
            } else {
                // NOTE 当中点值小于等于右边界时，说明中点到右边界都是递增的，所以mid有可能是最小元素，保留
                r = mid;
            }
        }
        return nums[l];
    }
}

#[test]
fn run() {
    assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
}
