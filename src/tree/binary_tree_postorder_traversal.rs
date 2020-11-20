// <二叉树的后序遍历>

// Given the root of a binary tree, return the postorder traversal of its nodes' values.

// Example 1:
//    1
//     \
//      2
//     /
//    3
// Input: root = [1,null,2,3]
// Output: [3,2,1]

// Example 2:
// Input: root = []
// Output: []

// Example 3:
//    1
// Input: root = [1]
// Output: [1]

// Example 4:
//    1
//   /
//  2
// Input: root = [1,2]
// Output: [2,1]

// Example 5:
//    1
//     \
//      2
// Input: root = [1,null,2]
// Output: [2,1]

// Constraints:
// The number of the nodes in the tree is in the range [0, 100].
// -100 <= Node.val <= 100

// Follow up:
// Recursive solution is trivial, could you do it iteratively?

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
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
            Self::recursive(node.borrow_mut().right.take(), result);
            result.push(node.borrow().val);
        }
    }

    /*
    循环：用栈模拟递归
    前序遍历：中->左->右
    后序遍历：左->右->中。可以将前序遍历改写为“中->右->左‘，再把该序列反序，形成后序遍历
    */
    fn loops(mut root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        while root.is_some() || !stack.is_empty() {
            // NOTE 循环递归右子树
            while let Some(node) = root {
                result.push(node.borrow().val);
                stack.push(node.clone());
                root = node.borrow_mut().right.take();
            }
            if let Some(node) = stack.pop() {
                root = node.borrow_mut().left.take();
            }
        }
        result.reverse();
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::postorder_traversal(TreeNode::example()),
        vec![2, 3, 1]
    );
}
