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
    /**
     * 遍历lists中所有元素，直到lists为空。构建新链表
     * 每次遍历找到最小的元素插入新链表，当遍历到链表为空时，从lists中删除
     * TODO 优化 可以用lists中的头结点构造一个最小堆，每次都取最小堆的堆顶插入新链表
     */
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        let mut result = Some(Box::new(ListNode::new(0)));
        let mut offset = &mut result;
        while !lists.is_empty() {
            let mut tmp = (0, i32::max_value());
            let mut i = 0;
            while i < lists.len() {
                if let Some(node) = lists[i].clone() {
                    if node.val < tmp.1 {
                        tmp = (i, node.val);
                    }
                    i += 1;
                } else {
                    lists.remove(i);
                }
            }
            if tmp.0 < lists.len() {
                let next = lists[tmp.0].as_mut().unwrap().next.take();
                offset.as_mut().unwrap().next = lists[tmp.0].take();
                offset = &mut offset.as_mut().unwrap().next;
                lists[tmp.0] = next;
            }
        }
        return result.unwrap().next;
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
