// <二叉树的最小深度>

// Given a binary tree, find its minimum depth.
// The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.
// Note: A leaf is a node with no children.

// Example 1:
//     3
//    / \
//   9  20
//      / \
//     15  7
// Input: root = [3,9,20,null,null,15,7]
// Output: 2

// Example 2:
// Input: root = [2,null,3,null,4,null,5,null,6]
// Output: 5

// Constraints:
// The number of nodes in the tree is in the range [0, 105].
// -1000 <= Node.val <= 1000

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left_deepth = Self::min_depth(node.borrow_mut().left.take());
            let right_deepth = Self::min_depth(node.borrow_mut().right.take());
            // NOTE 不能直接写“return 1 + min(left_deepth, right_deepth);”。左右孩子都为null时才算子树
            if left_deepth == 0 || right_deepth == 0 {
                return 1 + left_deepth + right_deepth;
            } else {
                return 1 + min(left_deepth, right_deepth);
            }
        }
        return 0;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::min_depth(TreeNode::example()), 2);
}
