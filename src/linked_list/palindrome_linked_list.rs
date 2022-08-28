// <回文链表>

// Given a singly linked list, determine if it is a palindrome.

// Example 1:
// Input: 1->2
// Output: false

// Example 2:
// Input: 1->2->2->1
// Output: true
// Follow up:
// Could you do it in O(n) time and O(1) space?

use crate::base::list_node::ListNode;
struct Solution;
impl Solution {
    /*
     * 快慢指针 + 反转链表
     * 通过快慢指针找到中点(注意判断慢指针的奇偶数) -> 反转慢指针之后的链表 -> 对比原链表和翻转后的新链表
     */
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut head = head;
        let mut fast = head.clone();
        let mut slow = &mut head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &mut slow.as_mut().unwrap().next;
            fast = fast.unwrap().next.unwrap().next;
        }
        let mut last = Self::reverse_loop(slow.take());
        while head.is_some() && last.is_some() {
            let h = head.unwrap();
            let l = last.unwrap();
            if h.val != l.val {
                return false;
            }
            head = h.next;
            last = l.next;
        }
        return true;
    }

    fn reverse_loop(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = None;
        while let Some(mut node) = head.take() {
            let next = node.next;
            node.next = new_head;
            new_head = Some(node);
            head = next;
        }
        return new_head;
    }
}
