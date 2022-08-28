// <删除链表中给定元素>
// Remove all elements from a linked list of integers that have value val.

// Example:
// Input:  1->2->6->3->4->5->6, val = 6
// Output: 1->2->3->4->5

use crate::base::list_node::ListNode;
struct Solution;
impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut tmp = &mut head;
        while tmp.is_some() {
            if tmp.as_ref().unwrap().val == val {
                *tmp = tmp.take().unwrap().next;
            } else {
                tmp = &mut tmp.as_mut().unwrap().next;
            }
        }
        return head;
    }
}
