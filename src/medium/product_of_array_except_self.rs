// <除自身以外数组的乘积>

// Given an array nums of n integers where n > 1,  return an array output such that output[i] is equal to the product of all the elements of nums except nums[i].

// Example:
// Input:  [1,2,3,4]
// Output: [24,12,8,6]

// Constraint: It's guaranteed that the product of the elements of any prefix or suffix of the array (including the whole array) fits in a 32 bit integer.

// Note: Please solve it without division and in O(n).
// Follow up:
// Could you solve it with constant space complexity? (The output array does not count as extra space for the purpose of space complexity analysis.)

struct Solution;
impl Solution {
    /**
     * 找规律
     * 1      2      3      4
     *        1     1*2   1*2*3
     * 2*3*4 3*4     4      1
     * 24    12      8      4
     */
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut tmp = vec![1; nums.len()];
        for i in 1..nums.len() {
            tmp[i] = nums[i - 1] * tmp[i - 1];
        }
        let mut n = 1;
        for i in (0..tmp.len()).rev() {
            tmp[i] = tmp[i] * n;
            n = n * nums[i];
        }
        return tmp;
    }

    /**
     * 除法(不符合题目要求)
     * 首先求得所有值的乘积，依次除以当前位置的元素
     */
    pub fn product_except_self2(mut nums: Vec<i32>) -> Vec<i32> {
        let mut max = 1;
        for n in &nums {
            max *= *n;
        }
        for i in 0..nums.len() {
            nums[i] = max / nums[i];
        }
        return nums;
    }

    /**
     * BF暴力破解(不符合题目要求O(n))
     */
    pub fn product_except_self3(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        for i in 0..nums.len() {
            let mut tmp = 1;
            for j in 0..nums.len() {
                if i != j {
                    tmp *= nums[j];
                }
            }
            result[i] = tmp;
        }
        return result;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::product_except_self(vec![1, 2, 3, 4]),
        vec![24, 12, 8, 6]
    );
}
