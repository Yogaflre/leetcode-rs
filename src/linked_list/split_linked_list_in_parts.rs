// <分割链表>

// Given the head of a singly linked list and an integer k, split the linked list into k consecutive linked list parts.
// The length of each part should be as equal as possible: no two parts should have a size differing by more than one. This may lead to some parts being null.
// The parts should be in the order of occurrence in the input list, and parts occurring earlier should always have a size greater than or equal to parts occurring later.
// Return an array of the k parts.

// Example 1:
// Input: head = [1,2,3], k = 5
// Output: [[1],[2],[3],[],[]]
// Explanation:
// The first element output[0] has output[0].val = 1, output[0].next = null.
// The last element output[4] is null, but its string representation as a ListNode is [].

// Example 2:
// Input: head = [1,2,3,4,5,6,7,8,9,10], k = 3
// Output: [[1,2,3,4],[5,6,7],[8,9,10]]
// Explanation:
// The input has been split into consecutive parts with size difference at most 1, and earlier parts are a larger size than the later parts.

// Constraints:
// The number of nodes in the list is in the range [0, 1000].
// 0 <= Node.val <= 1000
// 1 <= k <= 50

use crate::base::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn split_list_to_parts(
        mut head: Option<Box<ListNode>>,
        k: i32,
    ) -> Vec<Option<Box<ListNode>>> {
        let mut len = 0;
        let mut tmp = &head;
        while tmp.is_some() {
            len += 1;
            tmp = &tmp.as_ref().unwrap().next;
        }

        let size = len / k as usize;
        let mut extra = len % k as usize;

        let mut res = vec![None; k as usize];
        let mut index = 0;
        while head.is_some() {
            let mut t = &mut head;
            let more = if extra > 0 {
                extra -= 1;
                1
            } else {
                0
            };
            for _ in 0..(size + more - 1) {
                t = &mut t.as_mut().unwrap().next;
            }
            let next = t.as_mut().unwrap().next.take();
            res[index] = head.take();
            head = next;
            index += 1;
        }
        return res;
    }
}
