// <排序链表>
// Sort a linked list in O(n log n) time using constant space complexity.

// Example 1:
// Input: 4->2->1->3
// Output: 1->2->3->4

// Example 2:
// Input: -1->5->3->4->0
// Output: -1->0->3->4->5

// 解题思路
// NOTE 题目要求时间复杂度为O(nlogn)，挑选堆排序、快棑和归并排序
// 方法一：归并排序
//  在数组中归并排序的空间复杂度为O(n)，在链表中由于数据不连续，可以任意移动指针，所以空间复杂度也为O(1)
use crate::base::list_node::ListNode;
struct Solution;
impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        // NOTE 可变指针 + 不可变指针，绕过ownership
        unsafe {
            let mut slow: *mut Option<Box<ListNode>> = &mut head;
            let mut fast: *mut Option<Box<ListNode>> = &mut head;
            while (*fast).is_some() {
                slow = &mut (*slow).as_mut().unwrap().next;
                if (*fast).as_mut().unwrap().next.is_some() {
                    fast = &mut (*fast).as_mut().unwrap().next.as_mut().unwrap().next
                } else {
                    break;
                }
            }
            return match (head, (*slow).take()) {
                (None, None) => None,
                (l, None) => l,
                (None, r) => r,
                (l, r) => Self::merge(Self::sort_list(l), Self::sort_list(r)),
            };
        }
    }

    fn merge(l: Option<Box<ListNode>>, r: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l, r) {
            (None, None) => None,
            (l, None) => l,
            (None, r) => r,
            (Some(mut l), Some(mut r)) => {
                if l.val <= r.val {
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

    // assert_eq!(
    //     Solution::sort_list(ListNode::create(vec![-1, 5, 3, 4, 0])),
    //     ListNode::create(vec![-1, 0, 3, 4, 5])
    // );
}
