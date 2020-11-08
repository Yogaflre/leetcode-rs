// <左叶子之和>

// Find the sum of all left leaves in a given binary tree.

// Example:
//     3
//    / \
//   9  20
//     /  \
//    15   7
// There are two left leaves in the binary tree, with values 9 and 15 respectively. Return 24.

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    /*
    递归，检查左子树是不是叶子结点
     */
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let mut tmp = 0;
                // NOTE 左子树如果是叶子结点，符合题意；如果不是叶子结点则递归左子树
                if let Some(l) = node.borrow_mut().left.take() {
                    if l.borrow().left.is_none() && l.borrow().right.is_none() {
                        tmp = l.borrow().val;
                    } else {
                        tmp = Self::sum_of_left_leaves(Some(l));
                    }
                }
                // NOTE 计算完左子树还要递归右子树
                return tmp + Self::sum_of_left_leaves(node.borrow_mut().right.take());
            }
            None => return 0,
        }
    }
}

#[test]
fn run() {
    assert_eq!(Solution::sum_of_left_leaves(TreeNode::example()), 2);
}
