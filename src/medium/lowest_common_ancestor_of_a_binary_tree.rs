// <二叉树的最近公共祖先>

// Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.
// According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).”
// Given the following binary tree:  root = [3,5,1,6,2,0,8,null,null,7,4]

// Example 1:
// Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
// Output: 3
// Explanation: The LCA of nodes 5 and 1 is 3.

// Example 2:
// Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
// Output: 5
// Explanation: The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself according to the LCA definition.

// Note: All of the nodes' values will be unique. p and q are different and both values will exist in the binary tree.

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
// NOTE 因为每个值是唯一的，所以p和q使用值来模拟node，绕过ownership
impl Solution {
    /**
     * 递归遍历左右子树
     * 当节点等于p或q时，直接返回该节点；当节点为null时返回0(null)
     * 当节点不等于p或q时，递归遍历所有子树
     *  当左或右子树不为空时，返回不为空的子树(一个子数为空说明在另一个子数中存在p和q)
     *  当两个子树都不为空时，返回root节点(两个子树都不为空则说明root为公共祖先)
     */
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> i32 {
        if let Some(node) = root {
            if node.borrow().val == p || node.borrow().val == q {
                return node.borrow().val;
            }
            let left: i32 = Self::lowest_common_ancestor(node.borrow().left.clone(), p, q);
            let right: i32 = Self::lowest_common_ancestor(node.borrow().right.clone(), p, q);
            if left == 0 {
                return right;
            }
            if right == 0 {
                return left;
            }
            return node.borrow().val;
        } else {
            return 0;
        }
    }

    /**
     * 持久化路径
     * 分别寻找p和q节点的二叉树路径，并保存
     * 遍历两条路径，找到不同的节点
     */
    pub fn lowest_common_ancestor2(root: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> i32 {
        let mut ps: Vec<i32> = vec![];
        let mut qs: Vec<i32> = vec![];
        Self::search2(&root, p, &mut ps);
        Self::search2(&root, q, &mut qs);
        let mut i = 0;
        let mut j = 0;
        let mut res = 0;
        while i < ps.len() && j < qs.len() && ps[i] == qs[j] {
            res = ps[i];
            i += 1;
            j += 1;
        }
        return res;
    }

    fn search2(root: &Option<Rc<RefCell<TreeNode>>>, target: i32, paths: &mut Vec<i32>) -> bool {
        if let Some(node) = root {
            paths.push(node.borrow().val);
            if node.borrow().val == target
                || Self::search2(&node.borrow().left.clone(), target, paths)
                || Self::search2(&node.borrow().right.clone(), target, paths)
            {
                return true;
            }
            paths.pop();
        }
        return false;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::lowest_common_ancestor(TreeNode::example(), 2, 3),
        1
    );
}
