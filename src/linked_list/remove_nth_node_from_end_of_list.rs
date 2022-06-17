// <删除链表倒数第N个结点>
// Given a linked list, remove the n-th node from the end of list and return its head.

// Example:
// Given linked list: 1->2->3->4->5, and n = 2.
// After removing the second node from the end, the linked list becomes 1->2->3->5.

// Note:
// Given n will always be valid.

// Follow up:
// Could you do this in one pass?

use crate::base::list_node::ListNode;

struct Solution;
impl Solution {
    /*
     * 哨兵节点+双指针
     * 使用双指针，先由头指针走N个结点，再同时走到链表结束。此时第二个指针的位置则是需要被删除的结点
     */
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        unsafe {
            let mut tmp_head = Some(Box::new(ListNode::new(0)));
            tmp_head.as_mut().unwrap().next = head;
            let mut a: *const Option<Box<ListNode>> = &tmp_head;
            let mut b: *mut Option<Box<ListNode>> = &mut tmp_head;
            for _ in 0..n {
                a = &(*a).as_ref().unwrap().next;
            }
            loop {
                a = &(*a).as_ref().unwrap().next;
                if (*a).is_some() {
                    b = &mut (*b).as_mut().unwrap().next;
                } else {
                    break;
                }
            }
            (*b).as_mut().unwrap().next = (*b).as_mut().unwrap().next.as_mut().unwrap().next.take();
            return tmp_head.unwrap().next;
        }
    }
}

#[test]
fn run() {
    let head = ListNode::create(vec![1, 2, 3, 4, 5]);
    let result = ListNode::create(vec![1, 2, 3, 5]);
    assert_eq!(Solution::remove_nth_from_end(head, 2), result);

    let head = ListNode::create(vec![1]);
    let result = ListNode::create(vec![]);
    assert_eq!(Solution::remove_nth_from_end(head, 1), result);
}
