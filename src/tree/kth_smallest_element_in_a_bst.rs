// <二叉搜索树中第K小的元素>

// Given a binary search tree, write a function kthSmallest to find the kth smallest element in it.

// Example 1:
// Input: root = [3,1,4,null,2], k = 1
//    3
//   / \
//  1   4
//   \
//    2
// Output: 1

// Example 2:
// Input: root = [5,3,6,2,4,null,null,1], k = 3
//        5
//       / \
//      3   6
//     / \
//    2   4
//   /
//  1
// Output: 3
// Follow up:
// What if the BST is modified (insert/delete operations) often and you need to find the kth smallest frequently? How would you optimize the kthSmallest routine?

// Constraints:
// The number of elements of the BST is between 1 to 10^4.
// You may assume k is always valid, 1 ≤ k ≤ BST's total elements.

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    /**
     * 二叉搜索树特性：中序遍历为递增有序数组
     * 先递归终须遍历得到有序数组，取第k-1个值
     * 空间复杂度O(n)
     */
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut list: Vec<i32> = vec![];
        Self::recursive(root, &mut list);
        return list[k as usize - 1];
    }

    fn recursive(root: Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) {
        if let Some(node) = root {
            Self::recursive(node.borrow_mut().left.take(), list);
            list.push(node.borrow_mut().val);
            Self::recursive(node.borrow_mut().right.take(), list);
        }
    }

    /**
     * 计数
     * 空间复杂度O(1)
     */
    pub fn kth_smallest2(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut res = 0;
        Self::recursive2(root, k, &mut res);
        return res;
    }

    fn recursive2(root: Option<Rc<RefCell<TreeNode>>>, k: i32, res: &mut i32) -> i32 {
        if let Some(node) = root {
            // NOTE 递归左子树，计算当前节点树（二叉搜索树左子树最小）
            let l = 1 + Self::recursive2(node.borrow_mut().left.take(), k, res);
            if l == k {
                // NOTE 当左子树存在k时，将res赋值为当前节点的值
                *res = node.borrow().val;
            }
            // NOTE 如果左子树不存在则递归右子树查找（注意k的值在右子树也会发生变化）
            return l + Self::recursive2(node.borrow_mut().right.take(), k - l, res);
        } else {
            return 0;
        }
    }
}
