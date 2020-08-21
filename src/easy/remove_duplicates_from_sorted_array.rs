// <删除排序数组中的重复项>

// Given a sorted array nums, remove the duplicates in-place such that each element appear only once and return the new length.
// Do not allocate extra space for another array, you must do this by modifying the input array in-place with O(1) extra memory.

// Example 1:
// Given nums = [1,1,2],
// Your function should return length = 2, with the first two elements of nums being 1 and 2 respectively.
// It doesn't matter what you leave beyond the returned length.

// Example 2:
// Given nums = [0,0,1,1,1,2,2,3,3,4],
// Your function should return length = 5, with the first five elements of nums being modified to 0, 1, 2, 3, and 4 respectively.
// It doesn't matter what values are set beyond the returned length.

// Clarification:
// Confused why the returned value is an integer but your answer is an array?
// Note that the input array is passed in by reference, which means modification to the input array will be known to the caller as well.
// Internally you can think of this:

// // nums is passed in by reference. (i.e., without making a copy)
// int len = removeDuplicates(nums);

// // any modification to nums in your function would be known by the caller.
// // using the length returned by your function, it prints the first len elements.
// for (int i = 0; i < len; i++) {
//     print(nums[i]);
// }

struct Solution;
impl Solution {
    /**
     * 将不重复元素依次向前填充
     * 最后删除末尾的剩余元素
     */
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut index = 1;
        let mut tmp = nums[0];
        for i in 1..nums.len() {
            if nums[i] != tmp {
                nums[index] = nums[i];
                index += 1;
                tmp = nums[i];
            }
        }
        nums.split_off(index);
        return nums.len() as i32;
    }

    /**
     * 遍历，删除重复元素
     * 删除数组元素的时间复杂度是O(n)，总的时间复杂度为O(n^2)
     */
    pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut index = 1;
        let mut tmp = nums[0];
        while index < nums.len() {
            if nums[index] == tmp {
                nums.remove(index);
            } else {
                tmp = nums[index];
                index += 1;
            }
        }
        return nums.len() as i32;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::remove_duplicates(&mut vec![1, 1, 2]), 2);
    assert_eq!(
        Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
        5
    );
}
