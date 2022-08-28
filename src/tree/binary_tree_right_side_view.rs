// <二叉树的右视图>

// Given the root of a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.

// Example 1:
// Input: root = [1,2,3,null,5,null,4]
// Output: [1,3,4]

// Example 2:
// Input: root = [1,null,3]
// Output: [1,3]

// Example 3:
// Input: root = []
// Output: []

// Constraints:
// The number of nodes in the tree is in the range [0, 100].
// -100 <= Node.val <= 100
// Accepted
// 785,890
// Submissions

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::base::tree_node::TreeNode;

struct Solution;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if root.is_none() {
            return res;
        }
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let last_index = queue.len() - 1;
            res.push(queue[last_index].as_ref().unwrap().borrow().val);
            for _ in 0..=last_index {
                if let Some(node) = queue.pop_front().unwrap() {
                    if node.borrow().left.is_some() {
                        queue.push_back(node.borrow_mut().left.take());
                    }
                    if node.borrow().right.is_some() {
                        queue.push_back(node.borrow_mut().right.take());
                    }
                }
            }
        }
        return res;
    }
}
