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
        // return Self::reverse_list_loop(head);
        return Self::reverse_list_recursive(None, head);
    }

    /**
     * 在参数中传递pre节点
     */
    fn reverse_list_recursive(
        mut pre: Option<Box<ListNode>>,
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if head.is_some() {
            let next = head.as_mut().unwrap().next.take();
            head.as_mut().unwrap().next = pre;
            return Self::reverse_list_recursive(head, next);
        } else {
            return pre;
        }
    }

    fn reverse_list_loop(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head: Option<Box<ListNode>> = None;
        while let Some(mut node) = head.take() {
            let next: Option<Box<ListNode>> = node.next.take();
            node.next = new_head.take();

            new_head = Some(node);
            head = next;
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
