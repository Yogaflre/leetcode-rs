// <打家劫舍3>

// The thief has found himself a new place for his thievery again. There is only one entrance to this area, called the "root."
// Besides the root, each house has one and only one parent house. After a tour, the smart thief realized that "all houses in this place forms a binary tree".
// It will automatically contact the police if two directly-linked houses were broken into on the same night. Determine the maximum amount of money the thief can rob tonight without alerting the police.

// Example 1:
// Input: [3,2,3,null,3,null,1]
//      3
//     / \
//    2   3
//     \   \
//      3   1
// Output: 7
// Explanation: Maximum amount of money the thief can rob = 3 + 3 + 1 = 7.

// Example 2:
// Input: [3,4,5,1,3,null,1]
//      3
//     / \
//    4   5
//   / \   \
//  1   3   1
// Output: 9
// Explanation: Maximum amount of money the thief can rob = 4 + 5 = 9.

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

use std::cmp::max;
struct Solution;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_some() {
            return Self::recursive(&root);
        } else {
            return 0;
        }
    }

    /**
     * 递归
     * max(获取当前节点与跨层级节点的和,获取左右节点的和)
     * TODO DP：可以对TreeNode持久化中间结果()
     */
    fn recursive(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let mut tmp = node.borrow().val;
            if let Some(l) = &node.borrow().left {
                tmp += Self::recursive(&l.borrow().left) + Self::recursive(&l.borrow().right);
            }
            if let Some(r) = &node.borrow().right {
                tmp += Self::recursive(&r.borrow().left) + Self::recursive(&r.borrow().right);
            }
            return max(
                tmp,
                Self::recursive(&node.borrow().left) + Self::recursive(&node.borrow().right),
            );
        } else {
            return 0;
        }
    }
}
