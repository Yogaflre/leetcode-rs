// <二叉树最大路径和>

// Given a non-empty binary tree, find the maximum path sum.
// For this problem, a path is defined as any sequence of nodes from some starting node to any node in the tree along the parent-child connections. The path must contain at least one node and does not need to go through the root.

// Example 1:
// Input: [1,2,3]

//        1
//       / \
//      2   3
// Output: 6

// Example 2:
// Input: [-10,9,20,null,null,15,7]
//    -10
//    / \
//   9  20
//     /  \
//    15   7
// Output: 42

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Self::multi_pah(&root).unwrap_or(0);
    }

    fn multi_pah(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if let Some(node) = root {
            let ls = Self::single_path(&node.borrow().left)
                .filter(|n| *n > 0)
                .unwrap_or(0);
            let rs = Self::single_path(&node.borrow().right)
                .filter(|n| *n > 0)
                .unwrap_or(0);
            let tmp = node.borrow().val + ls + rs; // 当前节点最大路径

            let lm = Self::multi_pah(&node.borrow().left).unwrap_or(i32::min_value()); // 左字节点最大路径
            let rm = Self::multi_pah(&node.borrow().right).unwrap_or(i32::min_value()); // 右字节点最大路径

            return Some(max(tmp, max(lm, rm)));
        } else {
            return None;
        }
    }

    fn single_path(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if let Some(node) = root {
            let ls = Self::single_path(&node.borrow().left)
                .filter(|n| *n > 0)
                .unwrap_or(0);
            let rs = Self::single_path(&node.borrow().right)
                .filter(|n| *n > 0)
                .unwrap_or(0);
            let tmp = node.borrow().val + max(ls, rs);

            return Some(tmp);
        } else {
            return None;
        }
    }
}

#[test]
fn run() {
    assert_eq!(Solution::max_path_sum(TreeNode::example()), 6);
}
