// 单链表
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn create(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut result: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut temp: &mut Option<Box<ListNode>> = &mut result;
        for i in 0..nums.len() {
            temp.as_mut().unwrap().next = Some(Box::new(ListNode::new(nums[i])));
            if i < nums.len() {
                temp = &mut temp.as_mut().unwrap().next;
            }
        }
        result.unwrap().next
    }
}
