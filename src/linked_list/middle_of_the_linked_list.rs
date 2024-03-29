// <链表的中间结点>

// Given the head of a singly linked list, return the middle node of the linked list.
// If there are two middle nodes, return the second middle node.

// Example 1:
// Input: head = [1,2,3,4,5]
// Output: [3,4,5]
// Explanation: The middle node of the list is node 3.

// Example 2:
// Input: head = [1,2,3,4,5,6]
// Output: [4,5,6]
// Explanation: Since the list has two middle nodes with values 3 and 4, we return the second one.

// Constraints:
//     The number of nodes in the list is in the range [1, 100].
//     1 <= Node.val <= 100

use crate::base::list_node::ListNode;

struct Solution;
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &head;
        let mut slow = &head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
            slow = &slow.as_ref().unwrap().next;
        }
        return slow.clone();
    }
}
#[test]
fn run() {
    let head = ListNode::create(vec![1, 2, 3, 4]);
    let middle = Solution::middle_node(head);
    assert_eq!(middle.unwrap().val, 3);
}
