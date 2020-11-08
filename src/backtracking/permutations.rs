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

struct Solution;
impl Solution {
    /*
    递归
    遍历数组，每次取一个元素，并将除过这个元素剩下的元素进行递归
    当数组中元素等于2时，将该数组和该数组的反转均添加到结果集中
    */
    pub fn permute2(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 2 {
            return vec![nums];
        }
        let mut result: Vec<Vec<i32>> = vec![];
        Solution::recursive(&mut result, nums, vec![]);
        return result;
    }

    /*
    result：总结果集
    nums：未添加的数组
    tmp：已添加的数组
    NOTE 注意每一步骤都需要clone构建新的数组进行递归
    */
    fn recursive(result: &mut Vec<Vec<i32>>, nums: Vec<i32>, tmp: Vec<i32>) {
        // 当未添加的数组只剩两个元素时，将[a,b]和[b,a]均添加到tmp中，并且分别吧tmp添加到result中
        if nums.len() == 2 {
            let mut tmp1 = tmp.clone();
            tmp1.extend(nums.clone());
            result.push(tmp1);

            let mut tmp2 = tmp.clone();
            let mut new_nums = nums.clone();
            new_nums.reverse();
            tmp2.extend(new_nums);
            result.push(tmp2);
        } else {
            // 添加并删除当前遍历到的元素，递归剩下的元素
            for i in 0..nums.len() {
                let mut nums_copy = nums.clone();
                let mut tmp_copy = tmp.clone();
                nums_copy.remove(i);
                tmp_copy.push(nums[i]);
                Solution::recursive(result, nums_copy, tmp_copy);
            }
        }
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::permute2(vec![1, 2, 3]),
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
