// <>

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
use std::rc::Rc;
struct Solution;
impl Solution {
    /*
    TODO
    */
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        0
    }
}

#[test]
fn run() {
    assert_eq!(Solution::longest_univalue_path(TreeNode::example()), 0);
}
