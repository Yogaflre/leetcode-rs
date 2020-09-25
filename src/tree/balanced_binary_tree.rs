// <平衡二叉树>

// Given a binary tree, determine if it is height-balanced.
// For this problem, a height-balanced binary tree is defined as:
// a binary tree in which the left and right subtrees of every node differ in height by no more than 1.

// Example 1:
// Given the following tree [3,9,20,null,null,15,7]:
//     3
//    / \
//   9  20
//     /  \
//    15   7
// Return true.

// Example 2:
// Given the following tree [1,2,2,3,3,null,null,4,4]:

//        1
//       / \
//      2   2
//     / \
//    3   3
//   / \
//  4   4
// Return false.

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    /**
     * 递归
     * 求左右子树深度并判断高度。递归左右子树是否平衡
     */
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // TODO 完成
        false
    }

    /**
     * 求二叉树深度
     */
    fn depth() -> i32 {
        0
    }
}
