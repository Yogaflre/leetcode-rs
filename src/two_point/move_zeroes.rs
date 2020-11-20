// <移动零>

// Given an array nums, write a function to move all 0's to the end of it while maintaining the relative order of the non-zero elements.

// Example:
// Input: [0,1,0,3,12]
// Output: [1,3,12,0,0]

// Note:
// You must do this in-place without making a copy of the array.
// Minimize the total number of operations.

struct Solution;
impl Solution {
    /**
     * 双指针
     * 设置0的左右边界指针(l,r)，遍历数组
     * 当前值==0时，修改左右指针
     * 当前值!=0时，移动数据并修改左右指针
     */
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut l: usize = usize::max_value();
        let mut r: usize = l;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                if l == usize::max_value() {
                    l = i;
                } else {
                    r = i;
                }
            } else if l != usize::max_value() {
                nums.swap(i, l);
                if r != l {
                    l = l + 1;
                } else {
                    l = i;
                }
                r = i;
            }
        }
    }
}

#[test]
fn run() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, vec![1, 3, 12, 0, 0]);
}
