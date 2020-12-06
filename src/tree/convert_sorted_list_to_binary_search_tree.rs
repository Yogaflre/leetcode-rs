// <有序链表转换二叉搜索树>

// Given the head of a singly linked list where elements are sorted in ascending order, convert it to a height balanced BST.
// For this problem, a height-balanced binary tree is defined as a binary tree in which the depth of the two subtrees of every node never differ by more than 1.

// Example 1:
//       0
//      / \
//    -3   9
//    /   /
//  -10  5
// Input: head = [-10,-3,0,5,9]
// Output: [0,-3,9,-10,null,5]
// Explanation: One possible answer is [0,-3,9,-10,null,5], which represents the shown height balanced BST.

// Example 2:
// Input: head = []
// Output: []

// Example 3:
// Input: head = [0]
// Output: [0]

// Example 4:
// Input: head = [1,3]
// Output: [3,1]

// Constraints:
// The number of nodes in head is in the range [0, 2 * 104].
// -10^5 <= Node.val <= 10^5

use crate::base::list_node::ListNode;
use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if head.is_none() {
            return None;
        }
        return Self::creator(&head, &None);
    }

    // NOTE 必须指定首尾链表节点，用于递归标识当前链表的长度
    fn creator(
        head: &Option<Box<ListNode>>,
        tail: &Option<Box<ListNode>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if head == tail {
            return None;
        }
        // NOTE 利用快慢指针取链表的中点，再根据中点分别递归左右子链表
        let mut slow = head;
        let mut fast = head;
        while fast != tail && &fast.as_ref().unwrap().next != tail {
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
            slow = &slow.as_ref().unwrap().next;
        }
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(
            slow.as_ref().unwrap().val,
        ))));
        root.as_mut().unwrap().borrow_mut().left = Self::creator(&head, &slow);
        root.as_mut().unwrap().borrow_mut().right =
            Self::creator(&slow.as_ref().unwrap().next, tail);
        return root;
    }
}
