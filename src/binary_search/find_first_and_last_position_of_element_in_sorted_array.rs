// <查找排序数组中的第一个和最后一个位值>

// Given an array of integers nums sorted in ascending order, find the starting and ending position of a given target value.
// Your algorithm's runtime complexity must be in the order of O(log n).
// If the target is not found in the array, return [-1, -1].

// Example 1:
// Input: nums = [5,7,7,8,8,10], target = 8
// Output: [3,4]

// Example 2:
// Input: nums = [5,7,7,8,8,10], target = 6
// Output: [-1,-1]

struct Solution;
impl Solution {
    /**
     * 二分查找 + 遍历
     * 时间复杂度O(logn) ~ O(n)
     * 当nums[mid] == target时，分别向左向右遍历找到所有等于target的最远index
     */
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![-1, -1];
        let mut l: i32 = 0;
        let mut r: i32 = nums.len() as i32 - 1;
        while l <= r {
            let mid = l + (r - l) / 2;
            if nums[mid as usize] < target {
                l = mid + 1;
            } else if nums[mid as usize] > target {
                r = mid - 1;
            } else {
                let mut left = mid;
                let mut right = mid;
                while left >= 0 && nums[left as usize] == target {
                    res[0] = left;
                    left -= 1;
                }
                while right < nums.len() as i32 && nums[right as usize] == target {
                    res[1] = right;
                    right += 1;
                }
                return res;
            }
        }
        return res;
    }
}
#[test]
fn run() {
    assert_eq!(Solution::search_range(vec![1, 4], 4), vec![1, 1]);
    assert_eq!(Solution::search_range(vec![1], 1), vec![0, 0]);
    assert_eq!(Solution::search_range(vec![2, 2], 1), vec![-1, -1]);
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
        vec![3, 4]
    );
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
        vec![-1, -1]
    );
}
