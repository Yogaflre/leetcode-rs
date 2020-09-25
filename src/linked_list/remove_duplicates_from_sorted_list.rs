// <删除排序链表中的重复元素>

// Given a sorted linked list, delete all duplicates such that each element appear only once.

// Example 1:
// Input: 1->1->2
// Output: 1->2

// Example 2:
// Input: 1->1->2->3->3
// Output: 1->2->3

use crate::base::list_node::ListNode;
struct Solution;
impl Solution {
    /**
     * 如果next和当前节点相同，则把当前节点的next赋值为next.next
     */
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut tmp: &mut Box<ListNode> = head.as_mut().unwrap();
        while let Some(node) = tmp.next.as_mut() {
            if tmp.val == node.val {
                tmp.next = node.next.take();
            } else {
                tmp = tmp.next.as_mut().unwrap();
            }
        }
        return head;
    }
}
