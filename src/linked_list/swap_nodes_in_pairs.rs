// <两两交换链表中的节点>

// Given a linked list, swap every two adjacent nodes and return its head.
// You may not modify the values in the list's nodes, only nodes itself may be changed.

// Example:
// Given 1->2->3->4, you should return the list as 2->1->4->3.

use crate::base::list_node::ListNode;
struct Solution;
impl Solution {
    /**
     * 递归(精妙)
     * 每次修改前两个元素，然后递归后续节点
     */
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        return head.and_then(|mut node| match node.next {
            Some(mut next) => {
                node.next = Self::swap_pairs(next.next);
                next.next = Some(node);
                return Some(next);
            }
            None => Some(node),
        });
    }

    /**
     * 记录pre/tmp/next三个指针，交换tmp和next
     *  pre  tmp   next  
     *  0     1     2     3     4
     */
    pub fn swap_pairs2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node: Box<ListNode> = Box::new(ListNode::new(0));
        node.next = head;
        let mut pre = &mut node;
        while pre.next.is_some() {
            let mut tmp: Box<ListNode> = pre.next.take().unwrap();
            let mut next: Box<ListNode> = if tmp.next.is_some() {
                tmp.next.take().unwrap()
            } else {
                pre.next = Some(tmp);
                break;
            };

            tmp.next = next.next.take();
            next.next = Some(tmp);
            pre.next = Some(next);
            pre = pre.next.as_mut().unwrap().next.as_mut().unwrap();
        }
        return node.next;
    }
}
