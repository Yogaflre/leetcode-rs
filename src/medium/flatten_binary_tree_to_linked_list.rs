// <二叉树展开为链表>

// Given a binary tree, flatten it to a linked list in-place.

// For example, given the following tree:
//     1
//    / \
//   2   5
//  / \   \
// 3   4   6
// The flattened tree should look like:
// 1
//  \
//   2
//    \
//     3
//      \
//       4
//        \
//         5
//          \
//           6

// 解题思路
// 方法一：中序遍历
//  每次将左子树添加到根节点和右子树之间
use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_some() {
            Self::flatten(&mut root.as_ref().unwrap().borrow_mut().left);
            Self::flatten(&mut root.as_ref().unwrap().borrow_mut().right);
            if root.as_ref().unwrap().borrow().left.is_some() {
                let left: Option<Rc<RefCell<TreeNode>>> =
                    root.as_ref().unwrap().borrow_mut().left.take();
                let right = root.as_ref().unwrap().borrow_mut().right.take();
                root.as_ref().unwrap().borrow_mut().right = left;

                let mut tmp: *mut Option<Rc<RefCell<TreeNode>>> = root;
                // FIXME 可耻的unsafe了
                unsafe {
                    while (*tmp).as_ref().unwrap().borrow().right.is_some() {
                        tmp = &mut (*tmp).as_ref().unwrap().borrow_mut().right;
                    }
                    (*tmp).as_ref().unwrap().borrow_mut().right = right;
                }
            }
        }
    }
}
