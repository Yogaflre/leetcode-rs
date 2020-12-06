// <二叉搜索树的最小绝对差>

// Given a binary search tree with non-negative values, find the minimum absolute difference between values of any two nodes.

// Example:
// Input:
//    1
//     \
//      3
//     /
//    2
// Output:
// 1

// Explanation:
// The minimum absolute difference is 1, which is the difference between 2 and 1 (or between 2 and 3).
// Note:
// There are at least two nodes in this BST.

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // NOTE 设定两个递归变量：pre代表前一个节点值 res代表最终结果
        let mut res = i32::max_value();
        let mut pre = None;
        Self::inorder(root, &mut pre, &mut res);
        return res;
    }

    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, pre: &mut Option<i32>, res: &mut i32) {
        if let Some(node) = root {
            let val = node.borrow().val;
            Self::inorder(node.borrow_mut().left.take(), pre, res);
            // NOTE 当左子树为null时，root节点没有pre，所以需要使用Option来包裹，并判断是否有pre
            if let Some(p) = pre {
                let tmp = (*p - val).abs();
                if tmp < *res {
                    *res = tmp;
                }
            }
            *pre = Some(val);
            Self::inorder(node.borrow_mut().right.take(), pre, res);
        }
    }
}
