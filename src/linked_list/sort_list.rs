// <排序链表>
// Sort a linked list in O(n log n) time using constant space complexity.

// Example 1:
// Input: 4->2->1->3
// Output: 1->2->3->4

// Example 2:
// Input: -1->5->3->4->0
// Output: -1->0->3->4->5

use crate::base::list_node::ListNode;
struct Solution;
impl Solution {
    // 归并排序
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        unsafe {
            let mut slow: *mut Option<Box<ListNode>> = &mut head;
            let mut fast: *mut Option<Box<ListNode>> = &mut head;
            while (*fast).is_some() {
                slow = &mut (*slow).as_mut().unwrap().next; // slow的位置需要考虑
                if (*fast).as_ref().unwrap().next.is_none() {
                    break;
                }
                fast = &mut (*fast).as_mut().unwrap().next.as_mut().unwrap().next;
            }
            match (head, (*slow).take()) {
                (None, None) => None,
                (l, None) => l,
                (None, r) => r,
                (l, r) => Self::merge(Self::sort_list(l), Self::sort_list(r)),
            }
        }
    }

    fn merge(left: Option<Box<ListNode>>, right: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (left, right) {
            (None, None) => None,
            (l, None) => l,
            (None, r) => r,
            (Some(mut l), Some(mut r)) => {
                if l.val < r.val {
                    l.next = Self::merge(l.next.take(), Some(r));
                    return Some(l);
                } else {
                    r.next = Self::merge(Some(l), r.next.take());
                    return Some(r);
                }
            }
        }
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::sort_list(ListNode::create(vec![4, 2, 1, 3])),
        ListNode::create(vec![1, 2, 3, 4])
    );
}
