// <K 个一组翻转链表>

// Given the head of a linked list, reverse the nodes of the list k at a time, and return the modified list.
// k is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of k then left-out nodes, in the end, should remain as it is.
// You may not alter the values in the list's nodes, only nodes themselves may be changed.

// Example 1:
// Input: head = [1,2,3,4,5], k = 2
// Output: [2,1,4,3,5]

// Example 2:
// Input: head = [1,2,3,4,5], k = 3
// Output: [3,2,1,4,5]

// Constraints:
//     The number of nodes in the list is n.
//     1 <= k <= n <= 5000
//     0 <= Node.val <= 1000

// Follow-up: Can you solve the problem in O(1) extra memory space?

use crate::base::list_node::ListNode;

struct Solution;
impl Solution {
    // TODO 用unsafe方式完成
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        unsafe {}
        return head;
    }

    unsafe fn reverse(head: *mut Option<Box<ListNode>>) -> *mut Option<Box<ListNode>> {
        if (*head).is_none() || (*head).as_ref().unwrap().next.is_none() {
            return head;
        }
        let new_head = Self::reverse(&mut (*head).as_mut().unwrap().next);

        let next: *mut Option<Box<ListNode>> = &mut (*head).as_mut().unwrap().next;
        (*next).as_mut().unwrap().next = (*head).take();
        (*next).as_mut().unwrap().next = None;
        return new_head;
    }
}
