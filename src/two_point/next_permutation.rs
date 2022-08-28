// <下一个排列组合>

// Implement next permutation,
// which rearranges numbers into the lexicographically next greater permutation of numbers.

// If such arrangement is not possible,
// it must rearrange it as the lowest possible order (ie, sorted in ascending order).
// The replacement must be in-place and use only constant extra memory.

// Here are some examples.
// Inputs are in the left-hand column and its corresponding outputs are in the right-hand column.
// 1,2,3 → 1,3,2
// 3,2,1 → 1,2,3
// 1,1,5 → 1,5,1

struct Solution;
impl Solution {
    /*
     * 双指针
     * 遍历数组，找到左边 < 右边元素的位置(该位置表示转换大小的起点)，将左边的index设置为l
     * 遍历l+1..nums.len()的元素，寻找大于nums[l]的最小值(确定起始位置)，交换两个元素位置，并且对l之后元素进行排序
     */
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut l = 0;
        for i in (1..nums.len()).rev() {
            if nums[i - 1] < nums[i] {
                l = i - 1;
                break;
            }
        }
        let mut r = l;
        for i in l + 1..nums.len() {
            if nums[i] > nums[l] && (r == l || nums[i] < nums[r]) {
                r = i;
            }
        }
        if r != l {
            nums.swap(r, l);
            nums[l + 1..].sort();
        } else {
            nums.reverse();
        }
    }
}
