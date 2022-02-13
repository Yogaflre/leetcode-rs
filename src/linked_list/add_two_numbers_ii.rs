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
     * 不能反转链表的情况下，使用新队列来保存数值后计算
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

    /*
     * 1.反转链表
     * 2.遍历链表依次相加
     */
    pub fn add_two_numbers2(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        return Self::reverse_list(
            None,
            Self::add_two_number(Self::reverse_list(None, l1), Self::reverse_list(None, l2)),
        );
    }

    fn reverse_list(
        pre: Option<Box<ListNode>>,
        head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let Some(mut n) = head {
            let next = n.next.take();
            n.next = pre;
            return Self::reverse_list(Some(n), next);
        } else {
            return pre;
        }
    }

    fn add_two_number(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1.as_ref();
        let mut l2 = l2.as_ref();

        let mut root = ListNode::new(0);
        let mut carry = 0;
        let mut tmp = &mut root;
        while l1.is_some() || l2.is_some() {
            let v1: i32 = l1.map_or(0, |n| n.val);
            let v2: i32 = l2.map_or(0, |n| n.val);
            let sum = v1 + v2 + carry;
            //NOTE 记录进位值
            carry = sum / 10;
            //NOTE 取余数
            let node = Some(Box::new(ListNode::new(sum % 10)));
            tmp.next = node;

            tmp = tmp.next.as_mut().unwrap();
            l1 = l1.and_then(|n| n.next.as_ref());
            l2 = l2.and_then(|n| n.next.as_ref());
        }

        //NOTE 校验最后的进位
        if carry > 0 {
            let node = Some(Box::new(ListNode::new(carry)));
            tmp.next = node;
        }
        return root.next;
    }
}
