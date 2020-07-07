// <大小为 K 且平均值大于等于阈值的子数组数目>

// Given an array of integers arr and two integers k and threshold.
// Return the number of sub-arrays of size k and average greater than or equal to threshold.

// Example 1:
// Input: arr = [2,2,2,2,5,5,5,8], k = 3, threshold = 4
// Output: 3
// Explanation: Sub-arrays [2,5,5],[5,5,5] and [5,5,8] have averages 4, 5 and 6 respectively. All other sub-arrays of size 3 have averages less than 4 (the threshold).

// Example 2:
// Input: arr = [1,1,1,1,1], k = 1, threshold = 0
// Output: 5

// Example 3:
// Input: arr = [11,13,17,23,29,31,7,5,2,3], k = 3, threshold = 5
// Output: 6
// Explanation: The first 6 sub-arrays of size 3 have averages greater than 5. Note that averages are not integers.

// Example 4:
// Input: arr = [7,7,7,7,7,7,7], k = 7, threshold = 7
// Output: 1

// Example 5:
// Input: arr = [4,4,4,4], k = 4, threshold = 1
// Output: 1

// Constraints:
// 1 <= arr.length <= 10^5
// 1 <= arr[i] <= 10^4
// 1 <= k <= arr.length
// 0 <= threshold <= 10^4

struct Solution;
impl Solution {
    /**
     * 滑动窗口
     * 初始化求和sum，从初始值k的窗口开始滑动
     * 当值大于threshold，结果+1。sum减去即将离开窗口的值，并添加加入窗口的值，不断更新sum值进行比较
     */
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut r = k as usize - 1;
        let mut sum: i32 = arr[0..=r].iter().sum();
        let mut res = 0;
        let total = k * threshold;
        while r < arr.len() {
            if sum >= total {
                res += 1;
            }
            r += 1;
            if r < arr.len() {
                sum += arr[r] - arr[r - k as usize];
            }
        }
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::num_of_subarrays(vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4),
        3
    );
    assert_eq!(
        Solution::num_of_subarrays(vec![11, 13, 17, 23, 29, 31, 7, 5, 2, 3], 3, 5),
        6
    );
}
