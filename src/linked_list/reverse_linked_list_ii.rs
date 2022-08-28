// <反转链表 II>
// Given the head of a singly linked list and two integers left and right where left <= right, reverse the nodes of the list from position left to position right, and return the reversed list.

// Example 1:
// Input: head = [1,2,3,4,5], left = 2, right = 4
// Output: [1,4,3,2,5]

// Example 2:
// Input: head = [5], left = 1, right = 1
// Output: [5]

// Constraints:
// The number of nodes in the list is n.
// 1 <= n <= 500
// -500 <= Node.val <= 500
// 1 <= left <= right <= n

// Follow up: Could you do it in one pass?

use crate::base::list_node::ListNode;

struct Solution;

// TODO
impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        unsafe {
            let mut root = Some(Box::new(ListNode::new(0)));
            root.as_mut().unwrap().next = head;

            let mut prefix = &mut root as *mut Option<Box<ListNode>>;
            let mut suffix = &mut None as *mut Option<Box<ListNode>>;
            let mut node = &mut None as *mut Option<Box<ListNode>>;

            let mut i = 0;
            let mut tmp = &mut root as *mut Option<Box<ListNode>>;
            while i <= right {
                if i + 1 == left {
                    prefix = tmp;
                    node = &mut (*tmp).as_mut().unwrap().next;
                    tmp = &mut (*tmp).as_mut().unwrap().next;
                    i += 1;
                    continue;
                } else if i == right {
                    suffix = &mut (*tmp).as_mut().unwrap().next;
                    break;
                }
                tmp = &mut (*tmp).as_mut().unwrap().next;
                i += 1;
            }
            let reversed = Self::reverse(node);
            // println!("pre:{:?}", *pre);
            // println!("next:{:?}", *next);
            // println!("reversed:{:?}", reversed);
            (*node).as_mut().unwrap().next = (*suffix).take();
            (*prefix).as_mut().unwrap().next = reversed;
            return root.unwrap().next.take();
        }
    }

    unsafe fn reverse(mut head: *mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head: Option<Box<ListNode>> = None;
        while let Some(mut node) = (*head).take() {
            let next: *mut Option<Box<ListNode>> = &mut node.next.take();
            node.next = new_head.take();

            new_head = Some(node);
            head = next;
        }
        return new_head;
    }
}

#[test]
fn test() {
    assert_eq!(
        ListNode::create(vec![1, 2, 3]),
        Solution::reverse_between(ListNode::create(vec![1, 3, 2]), 2, 3)
    );
}
