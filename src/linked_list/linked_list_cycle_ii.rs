// <环形链表2>
// NOTE 本题leetcode没有Rust版本(Rust的所有权问题)，不过解题思路不变
// Given a linked list, return the node where the cycle begins. If there is no cycle, return null.
// To represent a cycle in the given linked list, we use an integer pos which represents the position (0-indexed) in the linked list where tail connects to. If pos is -1, then there is no cycle in the linked list.
// Note: Do not modify the linked list.

// Example 1:
// Input: head = [3,2,0,-4], pos = 1
// Output: tail connects to node index 1
// Explanation: There is a cycle in the linked list, where tail connects to the second node.

// Example 2:
// Input: head = [1,2], pos = 0
// Output: tail connects to node index 0
// Explanation: There is a cycle in the linked list, where tail connects to the first node.

// Example 3:
// Input: head = [1], pos = -1
// Output: no cycle
// Explanation: There is no cycle in the linked list.

// Follow-up:
// Can you solve it without using extra space?

use crate::base::list_node::ListNode;
struct Solution;
impl Solution {
    /*
     * 三指针+找规律(https://leetcode.com/problems/linked-list-cycle-ii/discuss/44774/Java-O(1)-space-solution-with-detailed-explanation.)
     * 首先用快慢指针找到相交的节点，由于快指针(f)的移动速度是慢指针(s)的两倍，所以快指针的移动距离为慢指针的两倍
     * 可以推导出慢指针再走到循环点的距离和从头走到循环点的距离相同(参见上面的题解)，所以新建tmp指针从头走，慢指针从相交点走，两个节点最终会相遇在循环点
     */
    pub fn detect_cycle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
            //快慢指针相遇
            if slow == fast {
                let mut tmp = &head;
                while tmp != slow {
                    slow = &slow.as_ref().unwrap().next;
                    tmp = &tmp.as_ref().unwrap().next;
                }
                return tmp.clone();
            }
        }
        return None;
    }
}
