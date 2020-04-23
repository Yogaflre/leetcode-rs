// <反转链表>
// Reverse a singly linked list.
// Example:
// Input: 1->2->3->4->5->NULL
// Output: 5->4->3->2->1->NULL
// Follow up:
// A linked list can be reversed either iteratively or recursively. Could you implement both?

use crate::base::list_node::ListNode;
struct Solution;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        return Self::reverse_list_loop(head);
        // return Self::reverse_list_recursive(head);
    }

    // TODO 递归链表反转
    fn reverse_list_recursive(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        None
    }

    fn reverse_list_loop(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut tmp = head;
        let mut new_head: Option<Box<ListNode>> = None;
        while let Some(mut node) = tmp.take() {
            let next = node.next;
            node.next = new_head.take();

            new_head = Some(node);
            tmp = next;
        }
        return new_head;
    }
}

#[test]
fn run() {
    let result = ListNode::create(vec![4, 3, 2, 1]);
    let root = ListNode::create(vec![1, 2, 3, 4]);

    assert_eq!(Solution::reverse_list(root), result);
}
