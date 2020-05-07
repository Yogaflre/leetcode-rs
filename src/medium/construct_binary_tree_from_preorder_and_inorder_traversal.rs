// <从前序与中序遍历序列构造二叉树>
// Given preorder and inorder traversal of a tree, construct the binary tree.
// Note:
// You may assume that duplicates do not exist in the tree.

// For example, given
// preorder = [3,9,20,15,7]
// inorder = [9,3,15,20,7]
// Return the following binary tree:
//     3
//    / \
//   9  20
//     /  \
//    15   7

// 解题思路
// 方法一：递归(了解前序遍历以及中序遍历特性)
//  前序遍历的第一个元素为root节点，值为val；在中序遍历val前半部分为左子树，右半部分为右子树。递归
use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::build_recursive(&preorder[..], &inorder[..]);
    }

    fn build_recursive(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        let root = Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))));
        let i_root: usize = inorder.iter().position(|&x| x == preorder[0]).unwrap();

        root.as_ref().unwrap().borrow_mut().left =
            Self::build_recursive(&preorder[1..=i_root], &inorder[..i_root]);
        root.as_ref().unwrap().borrow_mut().right =
            Self::build_recursive(&preorder[i_root + 1..], &inorder[i_root + 1..]);
        return root;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
        TreeNode::example()
    );
}
