// <删除链表倒数第N个结点>
// Given a linked list, remove the n-th node from the end of list and return its head.

// Example:
// Given linked list: 1->2->3->4->5, and n = 2.
// After removing the second node from the end, the linked list becomes 1->2->3->5.

// Note:
// Given n will always be valid.

// Follow up:
// Could you do this in one pass?

//解题思路
//方法一：
//  使用双指针，先由头指针走N个结点，再同时走到链表结束。此时第二个指针的位置则是需要被删除的结点
//方法二：重新构造链表（Safe）

use crate::base::list_node::ListNode;

struct Solution;
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        unsafe {
            let mut head = head;
            let mut first: *mut Option<Box<ListNode>> = &mut head;
            let mut second: *mut Option<Box<ListNode>> = &mut head;

            for _ in 0..n {
                first = &mut (*first).as_mut().unwrap().next;
            }

            if (*first).is_none() {
                return head.unwrap().next;
            }

            loop {
                first = &mut (*first).as_mut().unwrap().next;
                if (*first).is_none() {
                    break;
                }
                second = &mut (*second).as_mut().unwrap().next;
            }
            (*second).as_mut().unwrap().next = (*second)
                .as_mut()
                .unwrap()
                .next
                .as_mut()
                .unwrap()
                .next
                .take();
            head
        }
    }
}

#[test]
fn run() {
    let head = ListNode::create(vec![1, 2, 3, 4, 5]);
    let result = ListNode::create(vec![1, 2, 3, 5]);

    assert_eq!(Solution::remove_nth_from_end(head, 2), result);
}
