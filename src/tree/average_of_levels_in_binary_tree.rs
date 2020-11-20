// <二叉树的层平均值>

// Given a non-empty binary tree, return the average value of the nodes on each level in the form of an array.
// Example 1:
// Input:
//     3
//    / \
//   9  20
//     /  \
//    15   7
// Output: [3, 14.5, 11]
// Explanation:
// The average value of nodes on level 0 is 3,  on level 1 is 14.5, and on level 2 is 11. Hence return [3, 14.5, 11].
// Note:
// The range of node's value is in the range of 32-bit signed integer.

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;
struct Solution;
impl Solution {
    /*
    借助队列保存每一行二叉树的节点
     */
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut res: Vec<f64> = vec![];
        if root.is_none() {
            return res;
        }
        let mut list: LinkedList<Rc<RefCell<TreeNode>>> = LinkedList::new();
        list.push_back(root.unwrap());
        while !list.is_empty() {
            let level_num = list.len();
            let mut tmp: Vec<f64> = vec![];
            // NOTE 每次遍历一层的节点
            for _ in 0..level_num {
                if let Some(node) = list.pop_front() {
                    tmp.push(node.borrow().val as f64);
                    if let Some(l) = node.borrow_mut().left.take() {
                        list.push_back(l);
                    }
                    if let Some(r) = node.borrow_mut().right.take() {
                        list.push_back(r);
                    }
                }
            }
            res.push(tmp.iter().sum::<f64>() / tmp.len() as f64);
        }
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::average_of_levels(TreeNode::example()),
        vec![1_f64, 2.5_f64]
    );
}
