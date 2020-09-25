// <环形链表>
// NOTE 本题leetcode没有Rust版本(Rust的所有权问题)，不过解题思路不变
// Given a linked list, determine if it has a cycle in it.
// To represent a cycle in the given linked list, we use an integer pos which represents the position (0-indexed) in the linked list where tail connects to.
// If pos is -1, then there is no cycle in the linked list.

// Example 1:
// Input: head = [3,2,0,-4], pos = 1
// Output: true
// Explanation: There is a cycle in the linked list, where tail connects to the second node.

// Example 2:
// Input: head = [1,2], pos = 0
// Output: true
// Explanation: There is a cycle in the linked list, where tail connects to the first node.

// Example 3:
// Input: head = [1], pos = -1
// Output: false
// Explanation: There is no cycle in the linked list.

// Follow up:
// Can you solve it using O(1) (i.e. constant) memory?

use crate::base::list_node::ListNode;
struct Solution;
impl Solution {
    /**
     * 快慢指针
     * 一个指针走两步，一个指针走一步。直到指针节点为null或两个节点相等时为止
     */
    pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
        let mut a = &head;
        let mut b = &head;
        while b.is_some() && b.as_ref().unwrap().next.is_some() {
            a = &a.as_ref().unwrap().next;
            b = &b.as_ref().unwrap().next.as_ref().unwrap().next;
            if a == b {
                return true;
            }
        }
        return false;
    }
}
