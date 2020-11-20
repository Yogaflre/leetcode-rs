// <最长同值路径>

// Given the root of a binary tree, return the length of the longest path, where each node in the path has the same value. This path may or may not pass through the root.
// The length of the path between two nodes is represented by the number of edges between them.

// Example 1:
//      5
//     / \
//    4   5
//   / \   \
//  1  1    5
// Input: root = [5,4,5,1,1,5]
// Output: 2

// Example 2:
//      1
//     / \
//    4   5
//   / \   \
//  4  5    5
// Input: root = [1,4,5,4,4,5]
// Output: 2

// Constraints:
// The number of nodes in the tree is in the range [0, 104].
// -1000 <= Node.val <= 1000
// The depth of the tree will not exceed 1000.

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
struct Solution;
impl Solution {
    /*
    1.递归当前节点的最长路径
    2.递归左右子节点的最长路径
    */
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let tmp = Self::max_length(&node.borrow().left, node.borrow().val)
                    + Self::max_length(&node.borrow().right, node.borrow().val);

                let left = Self::longest_univalue_path(node.borrow().left.clone());
                let right = Self::longest_univalue_path(node.borrow().right.clone());
                // 比较所有情况，取最大值
                return max(tmp, max(left, right));
            }
            None => return 0,
        }
    }

    /*
    root: 当前节点
    pre: 上一个节点的值，用来判断是否相同
    */
    fn max_length(root: &Option<Rc<RefCell<TreeNode>>>, pre_val: i32) -> i32 {
        match root {
            Some(node) => {
                let tmp_val = node.borrow().val;
                if pre_val == tmp_val {
                    // NOTE 计算当前节点时，只能取左右子树的最长路径
                    return 1 + max(
                        Self::max_length(&node.borrow().left, pre_val),
                        Self::max_length(&node.borrow().right, pre_val),
                    );
                } else {
                    return 0;
                }
            }
            None => return 0,
        }
    }
}
