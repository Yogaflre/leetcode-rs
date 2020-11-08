// <反转二叉树>

// Invert a binary tree.

// Example:
// Input:
//      4
//    /   \
//   2     7
//  / \   / \
// 1   3 6   9
// Output:
//      4
//    /   \
//   7     2
//  / \   / \
// 9   6 3   1
// Trivia:
// This problem was inspired by this original tweet by Max Howell:
// Google: 90% of our engineers use the software you wrote (Homebrew), but you can’t invert a binary tree on a whiteboard so f*** off.

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    /**
     * 递归：将左子树赋值给右子树，右子树赋值给左子树
     */
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let mut root = root;
        let left = root.as_mut().unwrap().borrow_mut().left.take();
        let right = root.as_mut().unwrap().borrow_mut().right.take();
        root.as_mut().unwrap().borrow_mut().left = Self::invert_tree(right);
        root.as_mut().unwrap().borrow_mut().right = Self::invert_tree(left);
        return root;
    }
}
