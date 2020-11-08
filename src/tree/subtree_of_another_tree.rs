// <另一个树的子树>

// Given two non-empty binary trees s and t, check whether tree t has exactly the same structure and node values with a subtree of s. A subtree of s is a tree consists of a node in s and all of this node's descendants. The tree s could also be considered as a subtree of itself.

// Example 1:
// Given tree s:
//      3
//     / \
//    4   5
//   / \
//  1   2
// Given tree t:
//    4
//   / \
//  1   2
// Return true, because t has the same structure and node values with a subtree of s.

// Example 2:
// Given tree s:
//      3
//     / \
//    4   5
//   / \
//  1   2
//     /
//    0
// Given tree t:
//    4
//   / \
//  1   2
// Return false.
use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    /*
    坚固两个递归思路：
    1.检查当前节点是否为子树
    2.检查左右节点是否分别为子树
    */
    pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (s, t) {
            (Some(sn), Some(tn)) => {
                let tmp = Self::check(&Some(sn.clone()), &Some(tn.clone()));
                let children = Self::is_subtree(sn.borrow().left.clone(), Some(tn.clone()))
                    || Self::is_subtree(sn.borrow().right.clone(), Some(tn.clone()));
                return tmp || children;
            }
            (None, None) => return true,
            (_, _) => return false,
        }
    }

    fn check(s: &Option<Rc<RefCell<TreeNode>>>, t: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (s, t) {
            (Some(sn), Some(tn)) => {
                if sn.borrow().val == tn.borrow().val {
                    return Self::check(&sn.borrow().left, &tn.borrow().left)
                        && Self::check(&sn.borrow().right, &tn.borrow().right);
                } else {
                    return false;
                }
            }
            (None, None) => return true,
            (_, _) => return false,
        }
    }
}
