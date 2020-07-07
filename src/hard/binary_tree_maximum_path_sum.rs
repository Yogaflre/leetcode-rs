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
    /**
     * 递归
     * ①递归顶点
     * ②递归左右子树中的任意一个
     * TODO 优化
     */
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Self::search(&root).unwrap_or(0);
    }

    fn search(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if let Some(node) = root {
            let mut tmp = node.borrow().val;
            let l = Self::search_one_path(&node.borrow().left);
            if l.unwrap_or(0) > 0 {
                tmp += l.unwrap_or(0);
            }
            let r = Self::search_one_path(&node.borrow().right);
            if r.unwrap_or(0) > 0 {
                tmp += r.unwrap_or(0);
            }

            let left = Self::search(&node.borrow().left);
            let right = Self::search(&node.borrow().right);

            return Some(max(
                tmp,
                max(
                    left.unwrap_or(i32::min_value()),
                    right.unwrap_or(i32::min_value()),
                ),
            ));
        } else {
            return None;
        }
    }

    fn search_one_path(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if let Some(node) = root {
            let mut tmp = max(
                Self::search_one_path(&node.borrow().left).unwrap_or(i32::min_value()),
                Self::search_one_path(&node.borrow().right).unwrap_or(i32::min_value()),
            );
            if tmp == i32::min_value() {
                tmp = 0;
            }
            return Some(max(node.borrow().val, node.borrow().val + tmp));
        } else {
            return None;
        }
    }
}

#[test]
fn run() {
    assert_eq!(Solution::max_path_sum(TreeNode::example()), 6);
}
