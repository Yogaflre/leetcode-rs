// <颜色分类>

// Given an array nums with n objects colored red, white, or blue, sort them in-place so that objects of the same color are adjacent, with the colors in the order red, white, and blue.
// Here, we will use the integers 0, 1, and 2 to represent the color red, white, and blue respectively.

// Follow up:
// Could you solve this problem without using the library's sort function?
// Could you come up with a one-pass algorithm using only O(1) constant space?

// Example 1:
// Input: nums = [2,0,2,1,1,0]
// Output: [0,0,1,1,2,2]

// Example 2:
// Input: nums = [2,0,1]
// Output: [0,1,2]

// Example 3:
// Input: nums = [0]
// Output: [0]

// Example 4:
// Input: nums = [1]
// Output: [1]

// Constraints:
// n == nums.length
// 1 <= n <= 300
// nums[i] is 0, 1, or 2.

struct Solution;
impl Solution {
    /**
     * 三指针
     * l：0的最右端
     * r：2的最左端
     * i：从左往右遍历当前位置(i <= r)
     */
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut l: i32 = 0;
        let mut r: i32 = nums.len() as i32 - 1;
        let mut i: i32 = 0;
        // NOTE 必须兼容i==r的情况
        while i <= r {
            if nums[i as usize] == 0 {
                // NOTE 当nums[i]==0时，和l互换位置，根据判断i位置只能是1，所以l和i都+1
                nums.swap(i as usize, l as usize);
                l += 1;
                i += 1;
            } else if nums[i as usize] == 2 {
                // NOTE 当nums[i]==2时，和r互换位置，因为r位置可能为0/1/2，所以r-1，i不变
                nums.swap(i as usize, r as usize);
                r -= 1;
            } else {
                // NOTE 当nums[i]==1时，继续遍历下一个位置元素
                i += 1;
            }
        }
    }
}

#[test]
fn run() {
    let mut nums1 = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut nums1);
    assert_eq!(nums1, vec![0, 0, 1, 1, 2, 2]);

    let mut nums2 = vec![1, 2, 0];
    Solution::sort_colors(&mut nums2);
    assert_eq!(nums2, vec![0, 1, 2]);
}
