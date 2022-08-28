// <二叉树剪枝>

// Given the root of a binary tree, return the same tree where every subtree (of the given tree) not containing a 1 has been removed.
// A subtree of a node node is node plus every node that is a descendant of node.

// Example 1:
// Input: root = [1,null,0,0,1]
// Output: [1,null,0,null,1]
// Explanation:
// Only the red nodes satisfy the property "every subtree not containing a 1".
// The diagram on the right represents the answer.

// Example 2:
// Input: root = [1,0,1,0,0,0,1]
// Output: [1,null,1,null,1]

// Example 3:
// Input: root = [1,1,0,1,1,0,1,0]
// Output: [1,1,0,1,1,null,1]

// Constraints:
// The number of nodes in the tree is in the range [1, 200].
// Node.val is either 0 or 1.

struct Solution;

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let l = Self::prune_tree(node.borrow_mut().left.take());
            let r = Self::prune_tree(node.borrow_mut().right.take());
            if l.is_none() && r.is_none() && node.borrow().val == 0 {
                return None;
            } else {
                node.borrow_mut().left = l;
                node.borrow_mut().right = r;
                return Some(node);
            }
        } else {
            return None;
        }
    }
}
