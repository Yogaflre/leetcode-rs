// <二叉树的直径>

// Given a binary tree, you need to compute the length of the diameter of the tree. The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.

// Example:
// Given a binary tree
//           1
//          / \
//         2   3
//        / \
//       4   5
// Return 3, which is the length of the path [4,2,1,3] or [5,2,1,3].
// Note: The length of path between two nodes is represented by the number of edges between them.

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
struct Solution;
impl Solution {
    /**
     * 递归(深度优先遍历)
     * 分为三个递归部分：①当前左右结点最大长度②左节点作为root节点③右节点作为root节点
     */
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut tmp = 0;
        if let Some(node) = root {
            let left = Self::depth(&node.borrow().left);
            let right = Self::depth(&node.borrow().right);
            let l = Self::diameter_of_binary_tree(node.borrow_mut().left.take());
            let r = Self::diameter_of_binary_tree(node.borrow_mut().right.take());
            tmp = max(left + right, max(l, r));
        }
        return tmp;
    }

    /**
     * 求解二叉树深度
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
    assert_eq!(Solution::diameter_of_binary_tree(TreeNode::example()), 2);
}
