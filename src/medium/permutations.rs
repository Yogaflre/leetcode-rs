// <全排列>
// Given a collection of distinct integers, return all possible permutations.

// Example:
// Input: [1,2,3]
// Output:
// [
//   [1,2,3],
//   [1,3,2],
//   [2,1,3],
//   [2,3,1],
//   [3,1,2],
//   [3,2,1]
// ]

// 解题思路
// TODO 优化！太粗糙了
// 方法一：递归（写的有点儿笨）
//  遍历数组，每次取一个元素，并将除过这个元素剩下的元素进行递归。当数组中元素等于2时，将该数组和该数组的反转均添加到结果集中
struct Solution;
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 2 {
            return vec![nums];
        }
        let mut result: Vec<Vec<i32>> = vec![];
        Solution::recursive(&mut result, nums, vec![]);
        return result;
    }

    fn recursive(result: &mut Vec<Vec<i32>>, nums: Vec<i32>, tmp: Vec<i32>) {
        if nums.len() == 2 {
            let mut tmp1 = tmp.clone();
            tmp1.extend(nums.clone());
            result.push(tmp1);

            let mut tmp2 = tmp.clone();
            let mut new_nums = nums.clone();
            new_nums.reverse();
            tmp2.extend(new_nums);
            result.push(tmp2);
        }
        for i in 0..nums.len() {
            let mut nums_copy = nums.clone();
            let mut tmp_copy = tmp.clone();
            nums_copy.remove(i);
            tmp_copy.push(nums[i]);
            Solution::recursive(result, nums_copy, tmp_copy);
        }
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::permute(vec![1, 2, 3]),
        vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1]
        ]
    );
}
