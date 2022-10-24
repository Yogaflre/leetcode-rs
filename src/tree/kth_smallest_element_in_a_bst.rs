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
    /*
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

    /*
     * 计数
     * 空间复杂度O(1)
     */
    pub fn kth_smallest2(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        let mut res = 0;
        Self::recursive2(root, &mut k, &mut res);
        return res;
    }

    fn recursive2(root: Option<Rc<RefCell<TreeNode>>>, k: &mut i32, res: &mut i32) {
        if root.is_none() || *k == 0 {
            return;
        }
        let node = root.unwrap();
        Self::recursive2(node.borrow_mut().left.take(), k, res);
        *k -= 1;
        if *k == 0 {
            *res = node.borrow().val;
        }
        Self::recursive2(node.borrow_mut().right.take(), k, res);
    }
}
