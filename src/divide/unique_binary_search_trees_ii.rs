// <不同的二叉搜索树 II>

// Given an integer n, return all the structurally unique BST's (binary search trees), which has exactly n nodes of unique values from 1 to n. Return the answer in any order.

// Example 1:
// Input: n = 3
// Output: [[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]

// Example 2:
// Input: n = 1
// Output: [[1]]

// Constraints:
// 1 <= n <= 8

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        return Self::recursive(1, n as usize);
    }

    fn recursive(l: usize, r: usize) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut res = vec![];
        for i in l..=r {
            let ls = Self::recursive(l, i - 1);
            let rs = Self::recursive(i + 1, r);
            for l in ls.iter() {
                for r in rs.iter() {
                    let root = Some(Rc::new(RefCell::new(TreeNode::new(i as i32))));
                    if l.is_some() || r.is_some() {
                        root.as_ref().unwrap().borrow_mut().left = l.clone();
                        root.as_ref().unwrap().borrow_mut().right = r.clone();
                    }
                    res.push(root);
                }
            }
        }
        if res.is_empty() {
            res.push(None);
        }
        return res;
    }
}
