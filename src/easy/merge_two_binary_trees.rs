// <合并二叉树>

// Given two binary trees and imagine that when you put one of them to cover the other, some nodes of the two trees are overlapped while the others are not.
// You need to merge them into a new binary tree. The merge rule is that if two nodes overlap, then sum node values up as the new value of the merged node. Otherwise, the NOT null node will be used as the node of new tree.

// Example 1:
// Input:
// 	Tree 1                     Tree 2
//           1                         2
//          / \                       / \
//         3   2                     1   3
//        /                           \   \
//       5                             4   7
// Output:
// Merged tree:
// 	     3
// 	    / \
// 	   4   5
// 	  / \   \
// 	 5   4   7

// Note: The merging process must start from the root nodes of both trees.

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    /**
     * 递归
     * 以t1为主树，将t2合并到t1中
     * 若n1或n2为空，则返回另一个节点。否则将t2的值添加到t1中，并递归左右子树
     */
    pub fn merge_trees(
        t1: Option<Rc<RefCell<TreeNode>>>,
        t2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (t1, t2) {
            (Some(n1), Some(n2)) => {
                n1.borrow_mut().val += n2.borrow().val;
                let left = n1.borrow_mut().left.take();
                n1.borrow_mut().left = Self::merge_trees(left, n2.borrow_mut().left.take());
                let right = n1.borrow_mut().right.take();
                n1.borrow_mut().right = Self::merge_trees(right, n2.borrow_mut().right.take());
                return Some(n1);
            }
            (None, Some(n2)) => Some(n2),
            (Some(n1), None) => Some(n1),
            _ => None,
        }
    }
}
