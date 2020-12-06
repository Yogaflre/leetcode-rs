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
impl Solution {
    /**
     * 递归遍历左右子树
     * 当节点等于p或q时，直接返回该节点；当节点为null时返回0(null)
     * 当节点不等于p或q时，递归遍历所有子树
     *  当左或右子树不为空时，返回不为空的子树(一个子数为空说明在另一个子数中存在p和q)
     *  当两个子树都不为空时，返回root节点(两个子树都不为空则说明root为公共祖先)
     */
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::dfs(root, p.unwrap().borrow().val, q.unwrap().borrow().val);
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            // NOTE 当前节点等于p和q的任意一个时则返回当前节点（表示查找到了该节点）
            if node.borrow().val == p || node.borrow().val == q {
                return Some(node);
            } else {
                let left = Self::dfs(node.borrow_mut().left.take(), p, q);
                let right = Self::dfs(node.borrow_mut().right.take(), p, q);
                // NOTE 左右子树都查找到时返回root，任意一个为空则返回另一个节点
                if left == None {
                    return right;
                } else if right == None {
                    return left;
                } else {
                    return Some(node);
                }
            }
        } else {
            return None;
        }
    }

    /**
     * dfs：持久化路径
     */
    pub fn lowest_common_ancestor2(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut p_path = vec![];
        let mut q_path = vec![];
        Self::dfs2(&root, p.unwrap().borrow().val, &mut p_path);
        Self::dfs2(&root, q.unwrap().borrow().val, &mut q_path);
        let mut index = 0; 
        let mut res = 0; 
        while (index < p_path.len() && index < q_path.len() && p_path[index] == q_path[index]) {
            res = p_path[index];
            index += 1;
        }
        return Some(Rc::new(RefCell::new(TreeNode::new(res))));
    }

    fn dfs2(root: &Option<Rc<RefCell<TreeNode>>>, target: i32, paths: &mut Vec<i32>) -> bool {
        if let Some(node) = root {
            paths.push(node.borrow().val);
            if node.borrow().val == target
                || Self::dfs2(&node.borrow().left.clone(), target, paths)
                || Self::dfs2(&node.borrow().right.clone(), target, paths)
            {
                return true;
            }
            paths.pop();
        }
        return false;
    }
}
