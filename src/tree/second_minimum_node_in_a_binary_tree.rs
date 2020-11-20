// <二叉树中第二小的节点>

// Given a non-empty special binary tree consisting of nodes with the non-negative value, where each node in this tree has exactly two or zero sub-node. If the node has two sub-nodes, then this node's value is the smaller value among its two sub-nodes. More formally, the property root.val = min(root.left.val, root.right.val) always holds.
// Given such a binary tree, you need to output the second minimum value in the set made of all the nodes' value in the whole tree.
// If no such second minimum value exists, output -1 instead.

// Example 1:
//     2
//    / \
//   2  5
//     /  \
//    5   7
// Input: root = [2,2,5,null,null,5,7]
// Output: 5
// Explanation: The smallest value is 2, the second smallest value is 5.

// Example 2:
//    2
//   / \
//  2   2
// Input: root = [2,2,2]
// Output: -1
// Explanation: The smallest value is 2, but there isn't any second smallest value.

// Constraints:
// The number of nodes in the tree is in the range [1, 25].
// 1 <= Node.val <= 231 - 1
// root.val == min(root.left.val, root.right.val) for each internal node of the tree.

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::{max, min};
use std::rc::Rc;
struct Solution;
impl Solution {
    /*
    dfs
    */
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            match (left, right) {
                (Some(l), Some(r)) => {
                    let tv = node.borrow().val;
                    let mut lv = l.borrow().val;
                    let mut rv = r.borrow().val;
                    // NOTE 当左右子节点等于当前节点的值时，递归继续寻找最小值。否则当前值就是该子树的最小值
                    if tv == lv {
                        lv = Self::find_second_minimum_value(Some(l));
                    }
                    if tv == rv {
                        rv = Self::find_second_minimum_value(Some(r));
                    }
                    if lv == -1 || rv == -1 {
                        // NOTE -1一定是和根节点的值相同，所以我们取另一个节点的值
                        return max(lv, rv);
                    } else {
                        // NOTE 都不等于-1时，说明左右子树都有比根节点大的值，所以取最小值（必定大于根节点的值）
                        return min(lv, rv);
                    }
                }
                (_, _) => return -1,
            }
        } else {
            return -1;
        }
    }
}
