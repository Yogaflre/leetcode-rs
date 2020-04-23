// <合并两个有序链表>
// Merge two sorted linked lists and return it as a new list.
// The new list should be made by splicing together the nodes of the first two lists.

// Example:
// Input: 1->2->4, 1->3->4
// Output: 1->1->2->3->4->4

//解题思路
//方法一：构造新链表
//  1.初始化结果链表（result）和裸指针，设置l1、l2为可变链表
//  2.当l1、l2不为None时遍历，判断当前节点大小并将小节点写入result，并更新l1或l2节点
//  3.当l1或l2节点为None时，把剩下不为None的链表拼接到result后
//方法二：递归 *
//  1.构造"create_node"闭包，用来创建新链表
//  2.使用match匹配：
//      当l1和l2均有值时，判断大小加入新队列，并递归
//      当l1或l2为空时，将不为空的链表补全
use crate::base::list_node::ListNode;
struct Solution;
impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut result: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut index: &mut Option<Box<ListNode>> = &mut result;
        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                index.as_mut().unwrap().next =
                    Some(Box::new(ListNode::new(l1.as_ref().unwrap().val)));
                l1 = l1.unwrap().next;
            } else {
                index.as_mut().unwrap().next =
                    Some(Box::new(ListNode::new(l2.as_ref().unwrap().val)));
                l2 = l2.unwrap().next;
            }
            index = &mut index.as_mut().unwrap().next;
        }
        if l1.is_none() {
            index.as_mut().unwrap().next = l2;
        } else {
            index.as_mut().unwrap().next = l1;
        }
        return result.unwrap().next;
    }

    pub fn merge_two_lists2(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let create_node =
            |val: i32, next: Option<Box<ListNode>>| Some(Box::new(ListNode { val, next }));

        match (l1, l2) {
            (Some(n1), Some(n2)) => {
                if n1.val < n2.val {
                    create_node(n1.val, Self::merge_two_lists2(n1.next, Some(n2)))
                } else {
                    create_node(n2.val, Self::merge_two_lists2(Some(n1), n2.next))
                }
            }
            (Some(n1), _) => create_node(n1.val, n1.next),
            (_, Some(n2)) => create_node(n2.val, n2.next),
            _ => None,
        }
    }
}

#[test]
fn run() {
    let l1 = ListNode::create(vec![1, 2, 4]);
    let l2 = ListNode::create(vec![1, 3, 4]);
    assert_eq!(
        Solution::merge_two_lists2(l1, l2),
        ListNode::create(vec![1, 1, 2, 3, 4, 4])
    );
}
