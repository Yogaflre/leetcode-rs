// <两数之和 IV - 输入 BST>
// Given the root of a Binary Search Tree and a target number k, return true if there exist two elements in the BST such that their sum is equal to the given target.

// Example 1:
//     5
//    / \
//   3   6
//  / \   \
// 2   4   7
// Input: root = [5,3,6,2,4,null,7], k = 9
// Output: true
// Example 2:
//     5
//    / \
//   3   6
//  / \   \
// 2   4   7
// Input: root = [5,3,6,2,4,null,7], k = 28
// Output: false
// Example 3:
// Input: root = [2,1,3], k = 4
// Output: true
// Example 4:
// Input: root = [2,1,3], k = 1
// Output: false
// Example 5:
// Input: root = [2,1,3], k = 3
// Output: true

// Constraints:
// The number of nodes in the tree is in the range [1, 104].
// -104 <= Node.val <= 104
// root is guaranteed to be a valid binary search tree.
// -105 <= k <= 105

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    // 时间复杂度O(n) 空间复杂度O(n)
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut values = vec![];
        // NOTE 利用二叉搜索树特性中序遍历得到有序数组
        Self::inorder(root, &mut values);
        // 通过左右指针遍历是否存在目标值
        let mut l = 0;
        let mut r = values.len() - 1;
        while l < r {
            let tmp = values[l] + values[r];
            if tmp == k {
                return true;
            } else if tmp > k {
                r -= 1;
            } else {
                l += 1;
            }
        }
        return false;
    }

    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
        if let Some(node) = root {
            Self::inorder(node.borrow_mut().left.take(), values);
            values.push(node.borrow().val);
            Self::inorder(node.borrow_mut().right.take(), values);
        }
    }
}
