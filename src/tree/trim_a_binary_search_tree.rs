// <修剪二叉搜索树>

// Given the root of a binary search tree and the lowest and highest boundaries as low and high, trim the tree so that all its elements lies in [low, high]. Trimming the tree should not change the relative structure of the elements that will remain in the tree (i.e., any node's descendant should remain a descendant). It can be proven that there is a unique answer.
// Return the root of the trimmed binary search tree. Note that the root may change depending on the given bounds.

// Example 1:
// Input: root = [1,0,2], low = 1, high = 2
// Output: [1,null,2]

// Example 2:
// Input: root = [3,0,4,null,2,null,null,1], low = 1, high = 3
// Output: [3,2,null,1]

// Example 3:
// Input: root = [1], low = 1, high = 2
// Output: [1]

// Example 4:
// Input: root = [1,null,2], low = 1, high = 3
// Output: [1,null,2]

// Example 5:
// Input: root = [1,null,2], low = 2, high = 4
// Output: [2]

// Constraints:
// The number of nodes in the tree in the range [1, 104].
// 0 <= Node.val <= 104
// The value of each node in the tree is unique.
// root is guaranteed to be a valid binary search tree.
// 0 <= low <= high <= 104

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    /*
    递归校验当前节点大小是否符合要求。
    符合要求则递归左右子树
    小于low则返回递归右子树
    大于high则返回递归左子树
     */
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let tmp = node.borrow().val;
            if tmp >= low && tmp <= high {
                let l = Self::trim_bst(node.borrow_mut().left.take(), low, high);
                let r = Self::trim_bst(node.borrow_mut().right.take(), low, high);
                node.borrow_mut().left = l;
                node.borrow_mut().right = r;
                return Some(node);
            } else if tmp < low {
                return Self::trim_bst(node.borrow_mut().right.take(), low, high);
            } else {
                return Self::trim_bst(node.borrow_mut().left.take(), low, high);
            }
        } else {
            return None;
        }
    }
}

#[test]
fn run() {
    let root = Solution::trim_bst(TreeNode::example(), 1, 2);
    assert!(root.unwrap().borrow().right.is_none());
}
