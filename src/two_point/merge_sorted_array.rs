// <合并排序数组>

// Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as one sorted array.

// Note:
// The number of elements initialized in nums1 and nums2 are m and n respectively.
// You may assume that nums1 has enough space (size that is equal to m + n) to hold additional elements from nums2.

// Example:
// Input:
// nums1 = [1,2,3,0,0,0], m = 3
// nums2 = [2,5,6],       n = 3
// Output: [1,2,2,3,5,6]

struct Solution;
impl Solution {
    /**
     * 双指针
     * 设置三个索引位置(m:nums1最大值位置；n:nums2最大值位置；index:nums1尾部)
     * 依次比较m和n位置元素，并将大值添加到index位置
     * 最后再把n的剩余值添加到nums1中
     */
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m - 1;
        let mut n = n - 1;
        let mut index = nums1.len() as i32 - 1;
        while m >= 0 && n >= 0 {
            if nums1[m as usize] <= nums2[n as usize] {
                nums1[index as usize] = nums2[n as usize];
                n -= 1;
            } else {
                nums1[index as usize] = nums1[m as usize];
                m -= 1;
            }
            index -= 1;
        }
        // NOTE 防止nums2还没有遍历完的情况
        while n >= 0 {
            nums1[index as usize] = nums2[n as usize];
            n -= 1;
            index -= 1;
        }
    }

    /**
     * 将num2填充至num1后排序
     * 时间复杂度 O(nlogn)
     */
    pub fn merge2(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m;
        let mut index: usize = 0;
        while m < nums1.len() as i32 && index < n as usize {
            nums1[m as usize] = nums2[index];
            m += 1;
            index += 1;
        }
        nums1.sort();
    }
}

#[test]
fn run() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    Solution::merge(&mut nums1, 3, &mut nums2, 3);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);

    let mut nums3 = vec![4, 0, 0, 0, 0, 0];
    let mut nums4 = vec![1, 2, 3, 5, 6];
    Solution::merge(&mut nums3, 1, &mut nums4, 5);
    assert_eq!(nums3, vec![1, 2, 3, 4, 5, 6]);
}
