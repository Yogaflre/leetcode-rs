// <将有序数组转换为二叉搜索树>

//Given an array where elements are sorted in ascending order, convert it to a height balanced BST.
//For this problem, a height-balanced binary tree is defined as a binary tree in which the depth of the two subtrees of every node never differ by more than 1.

//Example:
//Given the sorted array: [-10,-3,0,5,9],
//One possible answer is: [0,-3,9,-10,null,5], which represents the following height balanced BST:
//
//      0
//     / \
//   -3   9
//   /   /
// -10  5


use std::rc::Rc;
use std::cell::RefCell;
use crate::base::tree_node::TreeNode;

struct Solution;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::creator(&nums, 0, nums.len() as i32 - 1);
    }

    // 因为是有序数组，所以每次寻找中间节点为root节点。并且递归左右数组构造二叉树
    fn creator(nums: &Vec<i32>, l: i32, r: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if l <= r {
            let mid = (l + r) / 2;
            let mut root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode::new(nums[mid as usize])))); 
            root.as_mut().unwrap().borrow_mut().left = Self::creator(nums, l, mid as i32 - 1);
            root.as_mut().unwrap().borrow_mut().right = Self::creator(nums, mid as i32 + 1, r);
            return root;
        } else {
            return None;
        } 
    }
}
