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
use std::cmp::max;
use std::rc::Rc;

struct Solution;
impl Solution {
    /**
     * 递归
     * 求左右子树深度并判断高度。递归左右子树是否平衡
     */
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            // NOTE 首先判断当前左右子树高度是否合法，当不合法时，还得继续递归左右子树当作跟节点判断是否合法
            if (Self::depth(&left) - Self::depth(&right)).abs() > 1 {
                return false;
            } else {
                return Self::is_balanced(left) && Self::is_balanced(right);
            }
        } else {
            return true;
        }
    }

    /**
     * 求二叉树深度
     */
    fn depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            return 1 + max(
                Self::depth(&node.borrow().left),
                Self::depth(&node.borrow().right),
            );
        } else {
            return 0;
        }
    }
}

#[test]
fn run() {
    assert_eq!(Solution::is_balanced(TreeNode::example()), true);
}
