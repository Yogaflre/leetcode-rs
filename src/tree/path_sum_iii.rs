// <路径总和3>

// You are given a binary tree in which each node contains an integer value.
// Find the number of paths that sum to a given value.
// The path does not need to start or end at the root or a leaf, but it must go downwards (traveling only from parent nodes to child nodes).
// The tree has no more than 1,000 nodes and the values are in the range -1,000,000 to 1,000,000.

// Example:
// root = [10,5,-3,3,2,null,11,3,-2,null,1], sum = 8

//       10
//      /  \
//     5   -3
//    / \    \
//   3   2   11
//  / \   \
// 3  -2   1
// Return 3. The paths that sum to 8 are:
// 1.  5 -> 3
// 2.  5 -> 2 -> 1
// 3. -3 -> 11

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    /**
     * dfs 深度优先遍历。需要分两种情况
     * 1.将左右节点分别当作root节点递归
     * 2.root节点递归计算sum
     */
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        match root {
            Some(node) => {
                let children_sum: i32 = Self::path_sum(node.borrow().left.clone(), sum)
                    + Self::path_sum(node.borrow().right.clone(), sum);
                let root_sum: i32 = Self::root_sum(&Some(node.clone()), sum);
                return children_sum + root_sum;
            }
            None => return 0,
        }
    }

    fn root_sum(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        match root {
            Some(node) => {
                let remainder = sum - node.borrow().val;
                let left = &node.borrow().left;
                let right = &node.borrow().right;
                // NOTE 当remainder==0时不代表结束，有可能子树正负相加还等于0
                let tmp = if remainder == 0 { 1 } else { 0 };
                return tmp + Self::root_sum(&left, remainder) + Self::root_sum(&right, remainder);
            }
            None => return 0,
        }
    }
}

#[test]
fn run() {
    assert_eq!(Solution::path_sum(TreeNode::example(), 3), 2);
}
