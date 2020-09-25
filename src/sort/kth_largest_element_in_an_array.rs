// <数组中第k个最大元素>

// Find the kth largest element in an unsorted array.
// Note that it is the kth largest element in the sorted order, not the kth distinct element.

// Example 1:
// Input: [3,2,1,5,6,4] and k = 2
// Output: 5

// Example 2:
// Input: [3,2,3,1,2,4,5,5,6] and k = 4
// Output: 4
// Note:
// You may assume k is always valid, 1 ≤ k ≤ array's length.

struct Solution;
impl Solution {
    /**
     * 排序
     * 由大到小排序，则第k+1个元素符合题意
     */
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut nums = nums;
        let right = nums.len() - 1;
        Self::fast(&mut nums, 0 as i32, right as i32);
        return nums[k as usize - 1];
    }

    /**
     * 实现快排(倒序排列)
     */
    fn fast(nums: &mut Vec<i32>, left: i32, right: i32) {
        if left >= right {
            return;
        }
        let left = left as usize;
        let right = right as usize;
        let mut l = left;
        let mut r = right;
        let mid = left;
        while l < r {
            while nums[r] <= nums[mid] && l < r {
                r -= 1;
            }
            while nums[l] >= nums[mid] && l < r {
                l += 1;
            }
            if l < r {
                nums.swap(l, r);
            }
        }
        nums.swap(mid, l);
        Self::fast(nums, left as i32, l as i32 - 1);
        Self::fast(nums, l as i32 + 1, right as i32);
    }
}

#[test]
fn run() {
    assert_eq!(Solution::find_kth_largest(vec![2, 1], 1), 2);
    assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    assert_eq!(
        Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
        4
    );
}
