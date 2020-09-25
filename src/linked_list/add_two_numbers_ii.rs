// <两数相加 II>

// You are given two non-empty linked lists representing two non-negative integers. The most significant digit comes first and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.
// You may assume the two numbers do not contain any leading zero, except the number 0 itself.

// Follow up:
// What if you cannot modify the input lists? In other words, reversing the lists is not allowed.

// Example:
// Input: (7 -> 2 -> 4 -> 3) + (5 -> 6 -> 4)
// Output: 7 -> 8 -> 0 -> 7

use crate::base::list_node::ListNode;
use std::collections::LinkedList;

struct Solution;
impl Solution {
    /**
     * 遍历获取所有链表的值，依次按位相加
     */
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        // NOTE 不能反转链表，则使用栈来存放链表中的值
        let mut one: LinkedList<i32> = LinkedList::new();
        let mut two: LinkedList<i32> = LinkedList::new();
        while l1.is_some() {
            one.push_back(l1.as_ref().unwrap().val);
            l1 = l1.unwrap().next;
        }
        while l2.is_some() {
            two.push_back(l2.as_ref().unwrap().val);
            l2 = l2.unwrap().next;
        }
        let mut head: Option<Box<ListNode>> = None;
        let mut add: i32 = 0;
        // NOTE 注意add != 0的情况
        while !one.is_empty() || !two.is_empty() || add != 0 {
            let a: i32 = if let Some(n) = one.pop_back() { n } else { 0 };
            let b: i32 = if let Some(n) = two.pop_back() { n } else { 0 };
            let mut tmp = a + b + add;
            if tmp > 9 {
                tmp = tmp % 10;
                add = 1;
            } else {
                add = 0;
            }
            // NOTE 因为前面反转链表相加，所以生成链表也需要倒序从头部添加
            let mut node = Some(Box::new(ListNode::new(tmp)));
            node.as_mut().unwrap().next = head;
            head = node;
        }
        return head;
    }
}
