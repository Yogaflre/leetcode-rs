// <打家劫舍3>

// The thief has found himself a new place for his thievery again. There is only one entrance to this area, called the "root."
// Besides the root, each house has one and only one parent house. After a tour, the smart thief realized that "all houses in this place forms a binary tree".
// It will automatically contact the police if two directly-linked houses were broken into on the same night. Determine the maximum amount of money the thief can rob tonight without alerting the police.

// Example 1:
// Input: [3,2,3,null,3,null,1]
//      3
//     / \
//    2   3
//     \   \
//      3   1
// Output: 7
// Explanation: Maximum amount of money the thief can rob = 3 + 3 + 1 = 7.

// Example 2:
// Input: [3,4,5,1,3,null,1]
//      3
//     / \
//    4   5
//   / \   \
//  1   3   1
// Output: 9
// Explanation: Maximum amount of money the thief can rob = 4 + 5 = 9.

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
struct Solution;
impl Solution {
    /*
    方法一：动态规划
    记录每个节点偷和不偷的最大值
     */
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let tuple = Self::dfs(&root);
        return max(tuple.0, tuple.1);
    }
    /*
    NOTE 每次递归返回该节点元组 1.偷当前节点的最大值 2.不偷当前节点的最大值
     */
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(node) = root {
            let tmp_val = node.borrow().val;
            let lt = Self::dfs(&node.borrow().left);
            let rt = Self::dfs(&node.borrow().right);
            // 当root节点偷，左右字节点就不能偷。直接 相加计算最大值
            let one = tmp_val + lt.1 + rt.1;
            // NOTE 当root节点不偷，需要分别对比左右字节点max(偷，不偷)再相加
            let two = max(lt.0, lt.1) + max(rt.0, rt.1);
            return (one, two);
        } else {
            return (0, 0);
        }
    }

    /*
    方法二：递归所有情况
    */
    pub fn rob2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Self::recursive(&root);
    }

    /**
     * 递归
     * max(获取当前节点与跨层级节点的和,获取左右节点的和)
     */
    fn recursive(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let a = node.borrow();
                let mut tmp = node.borrow().val;

                if let Some(l) = &node.borrow().left {
                    tmp += Self::recursive(&l.borrow().left) + Self::recursive(&l.borrow().right);
                }

                if let Some(r) = &node.borrow().right {
                    tmp += Self::recursive(&r.borrow().left) + Self::recursive(&r.borrow().right);
                }

                let amount = max(
                    tmp,
                    Self::recursive(&node.borrow().left) + Self::recursive(&node.borrow().right),
                );
                return amount;
            }
            None => return 0,
        }
    }
}
