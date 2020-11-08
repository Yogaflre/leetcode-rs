// <奇偶链表>

// Given a singly linked list, group all odd nodes together followed by the even nodes. Please note here we are talking about the node number and not the value in the nodes.
// You should try to do it in place. The program should run in O(1) space complexity and O(nodes) time complexity.

// Example 1:
// Input: 1->2->3->4->5->NULL
// Output: 1->3->5->2->4->NULL

// Example 2:
// Input: 2->1->3->5->6->4->7->NULL
// Output: 2->3->6->7->1->5->4->NULL
// Constraints:
// The relative order inside both the even and odd groups should remain as it was in the input.
// The first node is considered odd, the second node even and so on ...
// The length of the linked list is between [0, 10^4].

use crate::base::list_node::ListNode;
struct Solution;
impl Solution {
    /**
     * 构造两个独立的奇偶链表。依次填充两个链表，最终合并两个链表
     */
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odd = Some(Box::new(ListNode::new(0)));
        let mut even = Some(Box::new(ListNode::new(0)));
        let mut odd_tmp = &mut odd;
        let mut even_tmp = &mut even;
        // NOTE index来判断当前节点是奇结点还是偶结点
        let mut index = 1;
        while let Some(node) = head {
            if index % 2 == 0 {
                even_tmp.as_mut().unwrap().next = Some(node);
                even_tmp = &mut even_tmp.as_mut().unwrap().next;
                head = even_tmp.as_mut().unwrap().next.take();
            } else {
                odd_tmp.as_mut().unwrap().next = Some(node);
                odd_tmp = &mut odd_tmp.as_mut().unwrap().next;
                head = odd_tmp.as_mut().unwrap().next.take();
            }
            index += 1;
        }
        // NOTE 合并奇偶链表
        odd_tmp.as_mut().unwrap().next = even.as_mut().unwrap().next.take();
        return odd.as_mut().unwrap().next.take();
    }
}
