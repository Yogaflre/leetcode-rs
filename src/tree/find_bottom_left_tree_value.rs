// <找树左下角的值>

// Given a binary tree, find the leftmost value in the last row of the tree.

// Example 1:
// Input:
//     2
//    / \
//   1   3
// Output:
// 1

// Example 2:
// Input:
//         1
//        / \
//       2   3
//      /   / \
//     4   5   6
//        /
//       7
// Output:
// 7
// Note: You may assume the tree (i.e., the given root node) is not NULL.

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;

struct Solution;
impl Solution {
    /*
    bfs
     */
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut list: LinkedList<Rc<RefCell<TreeNode>>> = LinkedList::new();
        let mut res = 0;
        list.push_back(root.unwrap());
        while !list.is_empty() {
            let level_size = list.len();
            for i in 0..level_size {
                if let Some(node) = list.pop_front() {
                    if i == 0 {
                        // 当该层有节点时，取第一个节点的值
                        res = node.borrow().val;
                    }
                    if let Some(l) = node.borrow_mut().left.take() {
                        list.push_back(l);
                    }
                    if let Some(r) = node.borrow_mut().right.take() {
                        list.push_back(r);
                    }
                }
            }
        }
        return res;
    }
}
