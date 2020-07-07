// <滑动窗口最大值>

// Given an array nums, there is a sliding window of size k which is moving from the very left of the array to the very right.
// You can only see the k numbers in the window. Each time the sliding window moves right by one position. Return the max sliding window.

// Follow up:
// Could you solve it in linear time?

// Example:
// Input: nums = [1,3,-1,-3,5,3,6,7], and k = 3
// Output: [3,3,5,5,6,7]
// Explanation:
// Window position                Max
// ---------------               -----
// [1  3  -1] -3  5  3  6  7       3
//  1 [3  -1  -3] 5  3  6  7       3
//  1  3 [-1  -3  5] 3  6  7       5
//  1  3  -1 [-3  5  3] 6  7       5
//  1  3  -1  -3 [5  3  6] 7       6
//  1  3  -1  -3  5 [3  6  7]      7

// Constraints:
// 1 <= nums.length <= 10^5
// -10^4 <= nums[i] <= 10^4
// 1 <= k <= nums.length

struct Solution;
impl Solution {
    /**
     * 缓存最大值
     * 当前最大值为tmp。判断进入窗口和离开窗口的元素
     * 当nums[r]>tmp时，设置新的tmp，并添加到结果集
     * 当nums[l]<tmp时，说明当前最大值还在窗口内，直接添加tmp到结果集
     * 否则重新计算最大值
     */
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut max_values: Vec<i32> = vec![];
        let mut tmp = *nums[0..=(k as usize - 1)].iter().max().unwrap();
        max_values.push(tmp);
        let mut l: usize = 1;
        let mut r: usize = l + k as usize - 1;

        while r < nums.len() {
            if nums[r] > tmp {
                tmp = nums[r];
                max_values.push(tmp);
            } else if nums[l - 1] < tmp {
                max_values.push(tmp);
            } else {
                tmp = *nums[l..=r].iter().max().unwrap();
                max_values.push(tmp);
            }
            l += 1;
            r += 1;
        }
        return max_values;
    }

    /**
     * BP暴力法
     * 遍历所有区间，寻找最大值
     */
    pub fn max_sliding_window2(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut l: usize = 0;
        let mut r: usize = l + k as usize - 1;
        let mut max_values: Vec<i32> = vec![];
        while r < nums.len() {
            max_values.push(*nums[l..=r].iter().max().unwrap());
            l += 1;
            r += 1;
        }
        return max_values;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
        vec![3, 3, 5, 5, 6, 7]
    );
}
