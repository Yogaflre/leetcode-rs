// <寻找重复数>

// Given an array nums containing n + 1 integers where each integer is between 1 and n (inclusive), prove that at least one duplicate number must exist.
// Assume that there is only one duplicate number, find the duplicate one.

// Example 1:
// Input: [1,3,4,2,2]
// Output: 2

// Example 2:
// Input: [3,1,3,4,2]
// Output: 3

// Note:
// You must not modify the array (assume the array is read only).
// You must use only constant, O(1) extra space.
// Your runtime complexity should be less than O(n2).
// There is only one duplicate number in the array, but it could be repeated more than once.

struct Solution;
impl Solution {
    /**
     * 快慢指针(参见 linked_list::linked_list_cycle_ii)
     * 由于存在重复元素，把索引位置当做值遍历一定会形成环路
     * 先用快慢指针先找到相交点，再从头新建指针和慢指针一起遍历直到两个索引位置重合，该索引位置就是重复的元素
     */
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = 0;
        let mut fast = 0;
        loop {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
            if slow == fast {
                let mut tmp = 0;
                loop {
                    tmp = nums[tmp] as usize;
                    slow = nums[slow] as usize;
                    if tmp == slow {
                        return tmp as i32;
                    }
                }
            }
        }
    }
}

#[test]
fn run() {
    assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
}
