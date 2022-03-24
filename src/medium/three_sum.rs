// <三数之和>
// Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0?
// Find all unique triplets in the array which gives the sum of zero.

// Note:
// The solution set must not contain duplicate triplets.

// Example:
// Given array nums = [-1, 0, 1, 2, -1, -4],
// A solution set is:
// [
//   [-1, 0, 1],
//   [-1, -1, 2]
// ]

// 解题思路
// 方法一：双指针
//  1.对入参vec进行排序（主要是为了后续去重）
//  2.遍历vec（*注意*，需要对第二个重复元素进行过滤），并对i之后的元素取left=i+1；right=nums.len()-1作为两个指针
//  3.对i和两个指针位置的元素求sum：
//      当sum>0时，左移right指针
//      当sum<0时，右移left指针
//      当sum=0时，把现有指针指向的元素写入结果集，同时向中间移动两个指针（*注意*，当写入结果集后要对重复元素进行过滤）
//  4.最终返回结果集

struct Solution;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        let mut nums = nums;
        nums.sort();
        let mut result: Vec<Vec<i32>> = vec![];

        for i in 0..nums.len() {
            //跳过第一层重复
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum == 0 {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    //跳过第二层层重复
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }

                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
        return result;
    }

    /*
     * 1.排序
     * 2.优先确定前两个元素（过滤重复元素）
     * 3.二分查找最后一个元素
     */
    pub fn three_sum1(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res: Vec<Vec<i32>> = vec![];
        let length = nums.len();
        for i in 0..length {
            let ni = nums[i];
            if i > 0 && ni == nums[i - 1] {
                continue;
            }
            for j in (i + 1)..length {
                let nj = nums[j];
                if j > i + 1 && nj == nums[j - 1] {
                    continue;
                }
                let target = 0 - (ni + nj);
                if j + 1 < length && nums[j + 1..length].binary_search(&target).is_ok() {
                    res.push(vec![ni, nj, target]);
                }
            }
        }
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    );
    assert_eq!(
        Solution::three_sum(vec![-2, 0, 1, 1, 2]),
        vec![vec![-2, 0, 2], vec![-2, 1, 1]]
    );
    assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
}
