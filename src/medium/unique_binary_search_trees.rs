// <不同的二叉搜索树>
// Given n, how many structurally unique BST's (binary search trees) that store values 1 ... n?

// Example:
// Input: 3
// Output: 5
// Explanation:
// Given n = 3, there are a total of 5 unique BST's:

//    1         3     3      2      1
//     \       /     /      / \      \
//      3     2     1      1   3      2
//     /     /       \                 \
//    2     1         2                 3

// 解题思路
// 方法一：动态规划+找规律
//  f(1) = f(0) = 1;
//  f(2) = f(0)*f(1) + f(1)*f(0) = 2;
//  f(3) = f(0)*f(2) + f(1)*f(1) + f(2)*f(0) = 5;
//  f(4) = f(0)*f(3) + f(2)+f(1) + f(1)*f(2) + f(3)*f(0) = 42;
//  f(5) = f(0)*f(4) + f(1)*f(3) + f(2)*f(2) + f(3)*f(1) + f(4)*f(0) = 132;
struct Solution;
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut items = vec![1; n + 1];
        for i in 2..=n {
            items[i] = (0..i).map(|j| items[j] * items[i - j - 1]).sum();
        }
        return items[n];
    }
}

#[test]
fn run() {
    assert_eq!(Solution::num_trees(3), 5);
    assert_eq!(Solution::num_trees(4), 14);
    assert_eq!(Solution::num_trees(5), 42);
    assert_eq!(Solution::num_trees(6), 132);
}
