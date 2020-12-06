// <二叉搜索树中的众数>

// Given a binary search tree (BST) with duplicates, find all the mode(s) (the most frequently occurred element) in the given BST.
// Assume a BST is defined as follows:
// The left subtree of a node contains only nodes with keys less than or equal to the node's key.
// The right subtree of a node contains only nodes with keys greater than or equal to the node's key.
// Both the left and right subtrees must also be binary search trees.

// For example:
// Given BST [1,null,2,2],
//    1
//     \
//      2
//     /
//    2
// return [2].

// Note: If a tree has more than one mode, you can return them in any order.
// Follow up: Could you do that without using any extra space? (Assume that the implicit stack space incurred due to recursion does not count).

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    /*
     * 定义三个变量
     * res: 结果集
     * pre: 前驱值和前驱值的总数
     * count: 当前众数最大值
     */
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut pre: (Option<i32>, usize) = (None, 0);
        let mut count: usize = 0;
        Self::inorder(root, &mut res, &mut pre, &mut count);
        return res;
    }

    /*
     * 中序遍历：从小到大比较众数
     */
    fn inorder(
        root: Option<Rc<RefCell<TreeNode>>>,
        res: &mut Vec<i32>,
        pre: &mut (Option<i32>, usize),
        count: &mut usize,
    ) {
        if let Some(node) = root {
            Self::inorder(node.borrow_mut().left.take(), res, pre, count);
            Self::check(node.borrow().val, res, pre, count);
            Self::inorder(node.borrow_mut().right.take(), res, pre, count);
        }
    }

    /*
     * 遍历到每个节点都需要进行校验
     */
    fn check(val: i32, res: &mut Vec<i32>, pre: &mut (Option<i32>, usize), count: &mut usize) {
        // NOTE 如果前驱值不存在，则把当前值设为前驱值
        if pre.0.is_none() || *pre.0.as_ref().unwrap() != val {
            *pre = (Some(val), 0);
        }
        pre.1 += 1;
        if pre.1 == *count {
            // NOTE 如果当前值的总数等于最大值，说明众数重复，添加到res中
            res.push(*pre.0.as_ref().unwrap());
        } else if pre.1 > *count {
            // NOTE 当前值的总数大于最大值，则将res和count重新赋值
            res.clear();
            res.push(*pre.0.as_ref().unwrap());
            *count = pre.1;
        }
    }
}
