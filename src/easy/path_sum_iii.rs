// <路径总和3>

// You are given a binary tree in which each node contains an integer value.
// Find the number of paths that sum to a given value.
// The path does not need to start or end at the root or a leaf, but it must go downwards (traveling only from parent nodes to child nodes).
// The tree has no more than 1,000 nodes and the values are in the range -1,000,000 to 1,000,000.

// Example:
// root = [10,5,-3,3,2,null,11,3,-2,null,1], sum = 8

//       10
//      /  \
//     5   -3
//    / \    \
//   3   2   11
//  / \   \
// 3  -2   1
// Return 3. The paths that sum to 8 are:
// 1.  5 -> 3
// 2.  5 -> 2 -> 1
// 3. -3 -> 11

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    /**
     * dfs 深度优先遍历
     * 必须使用两个递归函数。如果写成一个递归函数会导致计算未连接的路径
     */
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        if let Some(node) = root {
            return Self::path_sum(node.borrow().left.clone(), sum)
                + Self::path_sum(node.borrow().right.clone(), sum)
                + Self::dfs(Some(node.clone()), sum);
        } else {
            return 0;
        }
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut res = 0;
        if let Some(node) = root {
            if node.borrow().val == sum {
                res += 1;
            }
            res += Self::dfs(node.borrow().left.clone(), sum - node.borrow().val)
                + Self::dfs(node.borrow().right.clone(), sum - node.borrow().val);
        }
        return res;
    }

    /**
     * ❌错误示范
     * WHY 会进行重复计算，但是为什么？
     */
    fn recursive(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut res: i32 = 0;
        if let Some(node) = root {
            if node.borrow().val == sum {
                res += 1;
            }
            res += Self::recursive(node.borrow().left.clone(), sum)
                + Self::recursive(node.borrow().right.clone(), sum);
            res += Self::recursive(node.borrow().left.clone(), sum - node.borrow().val)
                + Self::recursive(node.borrow().right.clone(), sum - node.borrow().val);
        }
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::path_sum(TreeNode::example(), 3), 2);
}
