use core::num;
use std::cell::RefCell;
use std::rc::Rc;

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

// 双向链表
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DoubleListNode {
    pub val: i32,
    pub pre: Option<Rc<RefCell<DoubleListNode>>>,
    pub next: Option<Rc<RefCell<DoubleListNode>>>,
}

// 双链表
impl DoubleListNode {
    pub fn new(val: i32) -> Self {
        DoubleListNode {
            val: val,
            pre: None,
            next: None,
        }
    }

    // TODO Rust实现双链表
    pub fn create(nums: Vec<i32>) -> Option<Rc<RefCell<DoubleListNode>>> {
        let root: Option<Rc<RefCell<DoubleListNode>>> =
            Some(Rc::new(RefCell::new(DoubleListNode::new(0))));
        if !nums.is_empty() {
            let mut pre = &root;
            for n in nums.into_iter() {
                pre.as_ref().unwrap().borrow_mut().next =
                    Some(Rc::new(RefCell::new(DoubleListNode::new(n))));
                pre.as_ref()
                    .unwrap()
                    .borrow()
                    .next
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .next = Some(pre.as_ref().unwrap().clone());
                // FIXME 怎么给pre赋新值呢？
            }
        }
        return root.unwrap().borrow_mut().next.take();
    }
}
