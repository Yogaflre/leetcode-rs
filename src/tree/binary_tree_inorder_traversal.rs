// <二叉树中序遍历>
// Given a binary tree, return the inorder traversal of its nodes' values.

// Example:
// Input: [1,null,2,3]
//    1
//     \
//      2
//     /
//    3

// Output: [1,3,2]
// Follow up: Recursive solution is trivial, could you do it iteratively?

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
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
            Self::recursive(node.borrow_mut().left.take(), result);
            result.push(node.borrow().val);
            Self::recursive(node.borrow_mut().right.take(), result);
        }
    }

    /*
    循环：使用栈模拟递归
     */
    fn loops(mut root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        // NOTE stack保存所有未记录的节点
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        while root.is_some() || !stack.is_empty() {
            // NOTE 只递归寻找左子树叶子节点，并添加到stack中
            while let Some(node) = root {
                stack.push(node.clone());
                root = node.borrow_mut().left.take();
            }
            // NOTE 记录当前节点值，并继续查找右子树
            if let Some(node) = stack.pop() {
                result.push(node.borrow().val);
                root = node.borrow_mut().right.take();
            }
        }
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::inorder_traversal(TreeNode::example()),
        vec![2, 1, 3]
    );
}
