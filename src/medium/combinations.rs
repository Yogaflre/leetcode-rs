// <组合>

// Given two integers n and k, return all possible combinations of k numbers out of 1 ... n.

// Example:
// Input: n = 4, k = 2
// Output:
// [
//   [2,4],
//   [3,4],
//   [2,3],
//   [1,2],
//   [1,3],
//   [1,4],
// ]

struct Solution;
impl Solution {
    /**
     * 回溯法
     * 从1开始遍历，构建临时tmp数组，递归前添加当前元素，递归后删除添加的元素
     * 递归终止条件：k=0。当k=0时候，tmp数组收集完毕
     */
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = vec![];
        let tmp: Vec<i32> = vec![];
        Self::recursive(1, n, k, tmp.clone(), &mut results);
        return results;
    }

    fn recursive(s: i32, n: i32, k: i32, mut tmp: Vec<i32>, results: &mut Vec<Vec<i32>>) {
        if k == 0 {
            results.push(tmp);
        } else {
            for i in s..=n {
                tmp.push(i);
                Self::recursive(i + 1, n, k - 1, tmp.clone(), results);
                tmp.pop();
            }
        }
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::combine(4, 2),
        vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4]
        ]
    );
}
