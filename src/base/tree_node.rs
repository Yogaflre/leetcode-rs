use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    // TODO 顺序存储转换为链式存储
    pub fn create(nodes: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode::new(10))));
        root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(14))));
        return root;
    }

    // 前序遍历
    pub fn preOrder(root: Option<Rc<RefCell<TreeNode>>>) {
        match root {
            Some(node) => {
                println!("node:{}", node.borrow().val);
                TreeNode::preOrder(node.borrow().left.clone());
                TreeNode::preOrder(node.borrow().right.clone());
            }
            None => (),
        }
    }

    // 使用栈，优先遍历左子树，将遍历完的左子树加入栈中，然后再判断右子树是否有值
    fn preOrderByLoop(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut node: Option<Rc<RefCell<TreeNode>>> = root;
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        let mut result: Vec<i32> = vec![];
        while node.is_some() || !stack.is_empty() {
            while let Some(n) = node {
                result.push(n.borrow().val);
                stack.push(Some(n.clone()));
                node = n.borrow().left.clone();
            }
            if let Some(n) = stack.pop() {
                node = n.unwrap().borrow().right.clone();
            }
        }
        return result;
    }
}

#[test]
fn run() {
    let nodes: Vec<i32> = vec![-1, 13, 10, 16, 9, 11, 14];
    let root: Option<Rc<RefCell<TreeNode>>> = TreeNode::create(nodes);
    // TreeNode::preOrder(root);
    TreeNode::preOrderByLoop(root);
    assert!(false);
}
