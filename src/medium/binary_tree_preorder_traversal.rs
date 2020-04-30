// <二叉树前序遍历>
// Given a binary tree, return the preorder traversal of its nodes' values.

// Example:
// Input: [1,null,2,3]
//    1
//     \
//      2
//     /
//    3

// Output: [1,2,3]
// Follow up: Recursive solution is trivial, could you do it iteratively?

// 解题思路
// 方法一：递归
// 方法二：循环(使用栈模拟递归)
use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        // Self::traversal_recursive(root, &mut result);
        Self::traversal_loop(root, &mut result);
        return result;
    }

    fn traversal_recursive(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            result.push(node.borrow().val);
            Self::traversal_recursive(node.borrow_mut().left.take(), result);
            Self::traversal_recursive(node.borrow_mut().right.take(), result);
        }
    }

    fn traversal_loop(mut root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        while !stack.is_empty() || root.is_some() {
            while let Some(node) = root {
                result.push(node.borrow().val);
                stack.push(node.clone());
                root = node.borrow_mut().left.take();
            }
            if let Some(node) = stack.pop() {
                root = node.borrow_mut().right.take();
            }
        }
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::preorder_traversal(TreeNode::example()),
        vec![1, 2, 3]
    );
}
