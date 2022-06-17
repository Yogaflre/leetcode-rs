// <最长重复子数组>

// Given two integer arrays nums1 and nums2, return the maximum length of a subarray that appears in both arrays.

// Example 1:
// Input: nums1 = [1,2,3,2,1], nums2 = [3,2,1,4,7]
// Output: 3
// Explanation: The repeated subarray with maximum length is [3,2,1].

// Example 2:
// Input: nums1 = [0,0,0,0,0], nums2 = [0,0,0,0,0]
// Output: 5

// Constraints:
//     1 <= nums1.length, nums2.length <= 1000
//     0 <= nums1[i], nums2[i] <= 100

struct Solution;
impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![-1; nums2.len()]; nums1.len()];
        let count = Self::recu(&nums1, &nums2, 0, 0, &mut dp);
        return count;
    }

    fn recu(nums1: &Vec<i32>, nums2: &Vec<i32>, i: usize, j: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if i == nums1.len() || j == nums2.len() {
            return 0;
        }
        if dp[i][j] != -1 {
            return dp[i][j];
        }
        let mut count = 0;

        // 比较两数组可优化
        let mut ti = i;
        let mut tj = j;
        while ti < nums1.len() && tj < nums2.len() && nums1[ti] == nums2[tj] {
            count += 1;
            ti += 1;
            tj += 1;
        }
        dp[i][j] = count;
        let sub_count = std::cmp::max(
            Self::recu(nums1, nums2, i + 1, j, dp),
            Self::recu(nums1, nums2, i, j + 1, dp),
        );
        count = std::cmp::max(count, sub_count);
        return count;
    }
}
