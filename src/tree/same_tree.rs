// <相同的树>

// Given the roots of two binary trees p and q, write a function to check if they are the same or not.
// Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.

// Example 1:
// Input: p = [1,2,3], q = [1,2,3]
// Output: true

// Example 2:
// Input: p = [1,2], q = [1,null,2]
// Output: false

// Example 3:
// Input: p = [1,2,1], q = [1,1,2]
// Output: false

// Constraints:
// The number of nodes in both trees is in the range [0, 100].
// -104 <= Node.val <= 104

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        if p.is_some() && q.is_some() {
            let pm = p.unwrap();
            let qm = q.unwrap();
            let ve = pm.borrow().val == qm.borrow().val;
            let le = Self::is_same_tree(pm.borrow_mut().left.take(), qm.borrow_mut().left.take());
            let re = Self::is_same_tree(pm.borrow_mut().right.take(), qm.borrow_mut().right.take());
            return ve && le && re;
        }
        return false;
    }
}
