// <恢复二叉搜索树>

// You are given the root of a binary search tree (BST), where the values of exactly two nodes of the tree were swapped by mistake. Recover the tree without changing its structure.

// Example 1:
// Input: root = [1,3,null,null,2]
// Output: [3,1,null,null,2]
// Explanation: 3 cannot be a left child of 1 because 3 > 1. Swapping 1 and 3 makes the BST valid.

// Example 2:
// Input: root = [3,1,4,null,null,2]
// Output: [2,1,4,null,null,3]
// Explanation: 2 cannot be in the right subtree of 3 because 2 < 3. Swapping 2 and 3 makes the BST valid.

// Constraints:
// The number of nodes in the tree is in the range [2, 1000].
// -231 <= Node.val <= 231 - 1

use std::cell::RefCell;
use std::rc::Rc;

use crate::base::tree_node::TreeNode;

struct Solution;
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let (mut node_a, mut node_b) = (None, None);
        let mut pre = None;
        Self::dfs(root, &mut pre, &mut node_a, &mut node_b);
        std::mem::swap(
            &mut node_a.unwrap().borrow_mut().val,
            &mut node_b.unwrap().borrow_mut().val,
        );
    }

    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Option<Rc<RefCell<TreeNode>>>,
        node_a: &mut Option<Rc<RefCell<TreeNode>>>,
        node_b: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(n) = node {
            let n_ref = n.borrow();
            Self::dfs(&n_ref.left, prev, node_a, node_b);
            if let Some(p) = prev {
                if p.borrow().val >= n.borrow().val {
                    if node_a.is_none() {
                        *node_a = Some(p.clone());
                    }
                    if node_a.is_some() {
                        *node_b = Some(n.clone());
                    }
                }
            }
            *prev = Some(n.clone());
            Self::dfs(&n_ref.right, prev, node_a, node_b);
        }
    }
}
