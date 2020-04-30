// <删除链表中给定元素>
// Remove all elements from a linked list of integers that have value val.

// Example:
// Input:  1->2->6->3->4->5->6, val = 6
// Output: 1->2->3->4->5

// 解题思路
// 方法一：遍历链表，如果当前节点等于val时，将next节点替换到当前节点
use crate::base::list_node::ListNode;
struct Solution;
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut tmp: &mut Option<Box<ListNode>> = &mut head;
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

#[test]
fn run() {
    assert_eq!(
        Solution::remove_elements(ListNode::create(vec![1, 1]), 1),
        ListNode::create(vec![])
    );
    assert_eq!(
        Solution::remove_elements(ListNode::create(vec![1, 2, 6, 3, 4, 5, 6]), 6),
        ListNode::create(vec![1, 2, 3, 4, 5])
    );
}
