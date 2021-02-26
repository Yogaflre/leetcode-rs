// <错误的集合>

// The set S originally contains numbers from 1 to n. But unfortunately, due to the data error, one of the numbers in the set got duplicated to another number in the set, which results in repetition of one number and loss of another number.
// Given an array nums representing the data status of this set after the error. Your task is to firstly find the number occurs twice and then find the number that is missing. Return them in the form of an array.

// Example 1:
// Input: nums = [1,2,2,4]
// Output: [2,3]

// Note:
// The given array size will in the range [2, 10000].
// The given array's numbers won't have any order.

struct Solution;
impl Solution {
    /*
     * 两次循环
     * 第一次寻找重复数字
     * 第二次寻找丢失的数字
     */
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut dup = 0;
        let mut miss = 1;
        let length = nums.len();
        // NOTE 将每个数字当作index，并将该位置的数字变为负数。如果该数已经为负数，则说明该index位置重复，并记录
        for i in 0..length {
            let index = nums[i].abs();
            let tmp = &mut nums[index as usize - 1];
            if *tmp < 0 {
                dup = index;
            } else {
                *tmp *= -1;
            }
        }
        // NOTE 找到第一个索引位置为>0的索引位置+1。因为除了重复数和丢失数，其他索引位置应该都为负数
        for i in 0..nums.len() {
            if nums[i] > 0 {
                miss = i as i32 + 1;
                break;
            }
        }
        return vec![dup, miss];
    }

    /*
     * 排序
     * 依次寻找前后非递增的数字
     */
    pub fn find_error_nums_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let mut dup = -1;
        let mut miss = 1;
        for i in 1..nums.len() {
            if nums[i - 1] == nums[i] {
                // NOTE 前后相等说明重复，记录重复的数字
                dup = nums[i];
            } else if nums[i - 1] + 1 != nums[i] {
                // NOTE 如果前一个数字+1 不等于后一个数字，则说明该数字是丢失的数字(存在特例)
                miss = nums[i - 1] + 1;
            }
        }
        if dup == 1 && miss == 1 {
            // NOTE 寻找miss数字存在特例情况，如果是[1, 1]则丢失的数字是2
            miss = 2;
        } else if nums[nums.len() - 1] != nums.len() as i32 {
            // NOTE 如果最后一个数字不等于该数组的长度，说明丢失的是最后一个数字
            miss = nums.len() as i32;
        }
        return vec![dup, miss];
    }
}

#[test]
fn run() {
    assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
    assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    assert_eq!(
        Solution::find_error_nums(vec![3, 2, 3, 4, 6, 5]),
        vec![3, 1]
    );
    assert_eq!(
        Solution::find_error_nums(vec![1, 5, 3, 2, 2, 7, 6, 4, 8, 9]),
        vec![2, 10]
    );
}
