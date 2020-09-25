// <二叉树最大深度>

// Given a binary tree, find its maximum depth.
// The maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
// Note: A leaf is a node with no children.

// Example:
// Given binary tree [3,9,20,null,null,15,7],

//     3
//    / \
//   9  20
//     /  \
//    15   7
// return its depth = 3.

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
struct Solution;
impl Solution {
    /**
     * 递归
     * 每次递归则+1深度
     */
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            return 1 + max(Self::max_depth(left), Self::max_depth(right));
        } else {
            return 0;
        }
    }
}

#[test]
fn run() {
    assert_eq!(Solution::max_depth(TreeNode::example()), 2);
}
