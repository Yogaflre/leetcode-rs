// <合并k个排序链表>
// Merge k sorted linked lists and return it as one sorted list. Analyze and describe its complexity.

// Example:
// Input:
// [
//   1->4->5,
//   1->3->4,
//   2->6
// ]
// Output: 1->1->2->3->4->4->5->6

// 解题思路
// 方法一：遍历
//  每次取所有链表最小的节点插入，再删除原链表中的该节点，直到vec为空
// 方法二：(非常耗时)
//  将合并多个链表分解为多次合并双链表

use crate::base::list_node::ListNode;
struct Solution;
impl Solution {
    //TODO find faster solution.
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        None
    }

    pub fn merge_k_lists2(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut result: Option<Box<ListNode>> = None;
        let mut index = 0;
        while index < lists.len() {
            match result {
                None => {
                    if lists.len() < 2 {
                        result = Self::merge_two_lists(&lists[index], &None);
                        return result;
                    } else {
                        result = Self::merge_two_lists(&lists[index], &lists[index + 1]);
                    }
                    index += 2;
                }
                _ => {
                    result = Self::merge_two_lists(&result, &lists[index]);
                    index += 1;
                }
            }
        }
        return result;
    }

    pub fn merge_two_lists(
        mut one: &Option<Box<ListNode>>,
        mut two: &Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut index = &mut result;
        while one.is_some() || two.is_some() {
            match (one.as_ref(), two.as_ref()) {
                (Some(n1), Some(n2)) => {
                    if n1.val < n2.val {
                        index.as_mut().unwrap().next = Some(Box::new(ListNode::new(n1.val)));
                        one = &one.as_ref().unwrap().next;
                    } else {
                        index.as_mut().unwrap().next = Some(Box::new(ListNode::new(n2.val)));
                        two = &two.as_ref().unwrap().next;
                    }
                }
                (Some(n1), None) => {
                    index.as_mut().unwrap().next = Some(Box::new(ListNode::new(n1.val)));
                    one = &one.as_ref().unwrap().next;
                }
                (None, Some(n2)) => {
                    index.as_mut().unwrap().next = Some(Box::new(ListNode::new(n2.val)));
                    two = &two.as_ref().unwrap().next;
                }
                _ => (),
            };
            index = &mut index.as_mut().unwrap().next;
        }
        return result.unwrap().next;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::merge_k_lists(vec![ListNode::create(vec![2]), ListNode::create(vec![-1])]),
        ListNode::create(vec![-1, 2])
    );
    assert_eq!(
        Solution::merge_k_lists(vec![ListNode::create(vec![])]),
        ListNode::create(vec![])
    );
    assert_eq!(
        Solution::merge_k_lists(vec![
            ListNode::create(vec![1, 4, 5]),
            ListNode::create(vec![1, 3, 4]),
            ListNode::create(vec![2, 6]),
        ]),
        ListNode::create(vec![1, 1, 2, 3, 4, 4, 5, 6])
    );
}
