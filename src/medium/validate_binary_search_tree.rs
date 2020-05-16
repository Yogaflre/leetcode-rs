// <验证二叉搜索树>

// Given a binary tree, determine if it is a valid binary search tree (BST).
// Assume a BST is defined as follows:
// The left subtree of a node contains only nodes with keys less than the node's key.
// The right subtree of a node contains only nodes with keys greater than the node's key.
// Both the left and right subtrees must also be binary search trees.

// Example 1:
//     2
//    / \
//   1   3

// Input: [2,1,3]
// Output: true

// Example 2:
//     5
//    / \
//   1   4
//      / \
//     3   6

// Input: [5,1,4,null,null,3,6]
// Output: false
// Explanation: The root node's value is 5 but its right child's value is 4.

// 解题思路
// 方法一：DFS + 找规律
//  通过二叉搜索树性质(左子树不能大于根节点和父节点，右子树不能小于根节点和父节点)。所以每个节点都需要进行最大值最小值校验
use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return Self::valid_recursive(root, i64::min_value(), i64::max_value());
    }

    fn valid_recursive(root: Option<Rc<RefCell<TreeNode>>>, min_val: i64, max_val: i64) -> bool {
        if let Some(node) = root {
            let val = node.borrow().val;
            if val as i64 <= min_val || val as i64 >= max_val {
                return false;
            } else {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                // 左子树的最大值设置为父节点值；右子树最小值设置为父节点值
                return Self::valid_recursive(left, min_val, val as i64)
                    && Self::valid_recursive(right, val as i64, max_val);
            }
        } else {
            return true;
        }
    }
}

#[test]
fn run() {
    assert_eq!(Solution::is_valid_bst(TreeNode::example()), false);
}
