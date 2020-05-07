// <二叉树层级遍历>
// Given a binary tree, return the level order traversal of its nodes' values. (ie, from left to right, level by level).

// For example:
// Given binary tree [3,9,20,null,null,15,7],
//     3
//    / \
//   9  20
//     /  \
//    15   7
// return its level order traversal as:
// [
//   [3],
//   [9,20],
//   [15,7]
// ]

// 解题思路
// 方法一：BFS(广度优先遍历) -> 队列

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        if root.is_none() {
            return result;
        }
        let mut list: LinkedList<Option<Rc<RefCell<TreeNode>>>> = LinkedList::new();
        list.push_front(root);
        let mut num = 1;
        while !list.is_empty() {
            let mut tmp: Vec<i32> = vec![];
            for _ in 0..num {
                if let Some(Some(node)) = list.pop_back() {
                    tmp.push(node.borrow().val);
                    let left = node.borrow_mut().left.take();
                    let right = node.borrow_mut().right.take();
                    if left.is_some() {
                        list.push_front(left);
                    }
                    if right.is_some() {
                        list.push_front(right);
                    }
                }
            }
            result.push(tmp.clone());
            num = list.len();
        }
        return result;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::level_order(TreeNode::example()),
        vec![vec![1], vec![2, 3]]
    );
}
