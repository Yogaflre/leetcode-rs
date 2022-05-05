use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// 前序遍历(tree::binary_tree_preorder_traversal)
// 中序遍历(tree::binary_tree_inorder_traversal)

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn example() -> Option<Rc<RefCell<TreeNode>>> {
        let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        return root;
    }
}
