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
     * 分为三个递归部分：①同时遍历左右节点②左节点作为root节点③右节点作为root节点
     * NOTE 不能写成一个递归函数，会重复计算
     */
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut tmp = 0;
        if let Some(node) = root {
            let mut l_and_r = 0;
            if node.borrow().left.is_some() {
                l_and_r += 1 + Self::search(&node.borrow().left);
            }
            if node.borrow().right.is_some() {
                l_and_r += 1 + Self::search(&node.borrow().right);
            }
            let l = Self::diameter_of_binary_tree(node.borrow_mut().left.take());
            let r = Self::diameter_of_binary_tree(node.borrow_mut().right.take());
            tmp = max(l_and_r, max(l, r));
        }
        return tmp;
    }

    fn search(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut tmp = 0;
        if let Some(node) = root {
            if node.borrow().left.is_some() {
                tmp += 1 + Self::search(&node.borrow().left);
            }
            if node.borrow().right.is_some() {
                tmp = max(tmp, 1 + Self::search(&node.borrow().right));
            }
        }
        return tmp;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::diameter_of_binary_tree(TreeNode::example()), 2);
}
