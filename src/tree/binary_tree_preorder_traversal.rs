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

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        // Self::recursive(root, &mut result);
        Self::loops(root, &mut result);
        return result;
    }

    /*
    递归
    */
    fn recursive(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            result.push(node.borrow().val);
            Self::recursive(node.borrow_mut().left.take(), result);
            Self::recursive(node.borrow_mut().right.take(), result);
        }
    }

    /*
    循环：使用栈模拟递归
    */
    fn loops(mut root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        // NOTE 栈用来保存未遍历右子树的所以节点
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        while root.is_some() || !stack.is_empty() {
            // NOTE 使用root作为当前节点，先递归所有的左子树，并将记录过的节点存入stack
            while let Some(node) = root {
                result.push(node.borrow().val);
                stack.push(node.clone());
                root = node.borrow_mut().left.take();
            }
            // NOTE 当左子树遍历完后，开始遍历stack中的右子树
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
