// 链表

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
                temp = &mut (*temp).as_mut().unwrap().next;
            }
        }
        result.unwrap().next
    }
}

// 双向链表
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DoubleListNode {
    pub pre: Option<Box<DoubleListNode>>,
    pub val: i32,
    pub next: Option<Box<DoubleListNode>>,
}

// 双链表
impl DoubleListNode {
    pub fn new(val: i32) -> Self {
        DoubleListNode {
            pre: None,
            val: val,
            next: None,
        }
    }

    // TODO Rust实现双链表
    pub fn create(nums: Vec<i32>) -> Option<Box<DoubleListNode>> {
        let mut result: Option<Box<DoubleListNode>> = Some(Box::new(DoubleListNode::new(0)));
        let mut temp: &mut Option<Box<DoubleListNode>> = &mut result;
        let mut pre: &mut Option<Box<DoubleListNode>> = &mut None;
        for i in 0..nums.len() {
            if i != 0 {
                // temp.as_mut().unwrap().pre = pre;
            }
            temp.as_mut().unwrap().next = Some(Box::new(DoubleListNode::new(nums[i])));
            temp = &mut (*temp).as_mut().unwrap().next;
            pre = temp;
        }
        return result.unwrap().next;
    }
}
