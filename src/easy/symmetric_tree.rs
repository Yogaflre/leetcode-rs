// <对称树>
// Given a binary tree, check whether it is a mirror of itself (ie, symmetric around its center).

// For example, this binary tree [1,2,2,3,4,4,3] is symmetric:
//     1
//    / \
//   2   2
//  / \ / \
// 3  4 4  3

// But the following [1,2,2,null,3,null,3] is not:
//     1
//    / \
//   2   2
//    \   \
//    3    3
// Follow up: Solve it both recursively and iteratively.

// 解题思路
// 方法一：递归
//  解题思路：递归左右节点，判断left.left == right.right && left.right == right.left;
// 方法二：循环
//  解题思路：递归->循环，用栈模拟递归
use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_some() {
            return Self::symmetric_recursive(
                &root.as_ref().unwrap().borrow().left,
                &root.as_ref().unwrap().borrow().right,
            );
        }
        return true;

        // return Self::symmetric_loop(root);
    }

    fn symmetric_recursive(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        return match (left, right) {
            (Some(l), Some(r)) => {
                l.borrow().val == r.borrow().val
                    && Self::symmetric_recursive(&l.borrow().left, &r.borrow().right)
                    && Self::symmetric_recursive(&l.borrow().right, &r.borrow().left)
            }
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (None, None) => true,
        };
    }

    fn symmetric_loop(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        if let Some(node) = root {
            stack.push(node.borrow_mut().left.take());
            stack.push(node.borrow_mut().right.take());
            while !stack.is_empty() {
                let right: Option<Rc<RefCell<TreeNode>>> = stack.pop().unwrap();
                let left: Option<Rc<RefCell<TreeNode>>> = stack.pop().unwrap();
                if left.is_some() && right.is_some() {
                    if left.as_ref().unwrap().borrow().val != right.as_ref().unwrap().borrow().val {
                        return false;
                    } else {
                        stack.push(left.as_ref().unwrap().borrow_mut().left.take());
                        stack.push(right.as_ref().unwrap().borrow_mut().right.take());
                        stack.push(left.as_ref().unwrap().borrow_mut().right.take());
                        stack.push(right.as_ref().unwrap().borrow_mut().left.take());
                    }
                } else if left.is_some() || right.is_some() {
                    return false;
                }
            }
        }
        return true;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::is_symmetric(TreeNode::example()), false);
}
