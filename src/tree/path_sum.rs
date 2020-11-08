// <路径总和>

// Given a binary tree and a sum, determine if the tree has a root-to-leaf path such that adding up all the values along the path equals the given sum.
// Note: A leaf is a node with no children.

// Example:
// Given the below binary tree and sum = 22,
//       5
//      / \
//     4   8
//    /   / \
//   11  13  4
//  /  \      \
// 7    2      1
// return true, as there exist a root-to-leaf path 5->4->11->2 which sum is 22.

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    /*
    递归左右子树，每次递归减少sum的值
     */
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        match root {
            Some(node) => {
                let remainder: i32 = sum - node.borrow().val;
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                if remainder == 0 && left.is_none() && right.is_none() {
                    return true;
                } else {
                    return Solution::has_path_sum(left, remainder)
                        || Solution::has_path_sum(right, remainder);
                }
            }
            // NOTE 当root为null时，不能简单的通过sum==0来判断（空树不能计算sum大小）
            None => return false,
        }
    }
}

#[test]
fn run() {
    assert_eq!(Solution::has_path_sum(TreeNode::example(), 3), true);
    assert_eq!(Solution::has_path_sum(TreeNode::example(), 4), true);
    assert_eq!(Solution::has_path_sum(TreeNode::example(), 1), false);
}
