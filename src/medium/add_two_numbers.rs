// <两数相加>
// You are given two non-empty linked lists representing two non-negative integers.
//  The digits are stored in reverse order and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.
// You may assume the two numbers do not contain any leading zero, except the number 0 itself.

// Example:
// Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
// Output: 7 -> 0 -> 8
// Explanation: 342 + 465 = 807.

// 解题思路
// 方法一：（位溢出风险）
//  先把两个初始链表构造为数字，再把两个数字相加（两个数字很可能超过整型最大值，导致溢出奉献），再把两数之和遍历得到新链表
// 方法二：
//  同时循环两个链表，取出对应位置的节点相加，并记录是否进位。**难点在于Ownership，需要理解其中as_ref/&等引用类型**

use crate::base::list_node::ListNode;
struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1.as_ref();
        let mut l2 = l2.as_ref();
        let mut carry = 0;
        let mut root = ListNode::new(0);
        let mut temp = &mut root;
        while l1.is_some() || l2.is_some() {
            let v1: i32 = l1.map_or(0, |n| n.val);
            let v2: i32 = l2.map_or(0, |n| n.val);
            let sum = v1 + v2 + carry;
            carry = sum / 10;
            temp.next = Some(Box::new(ListNode::new(sum % 10)));
            temp = temp.next.as_mut().unwrap();
            l1 = l1.and_then(|n| n.next.as_ref());
            l2 = l2.and_then(|n| n.next.as_ref());
        }
        if carry > 0 {
            temp.next = Some(Box::new(ListNode::new(carry)));
        }
        return root.next;
    }
}

#[test]
fn run() {
    let l1 = ListNode::create(vec![9]);
    let l2 = ListNode::create(vec![1, 9, 9]);
    let result = ListNode::create(vec![0, 0, 0, 1]);
    assert_eq!(Solution::add_two_numbers(l1, l2), result);
}
