// <二叉树的序列化与反序列化>

// Serialization is the process of converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.
// Design an algorithm to serialize and deserialize a binary tree. There is no restriction on how your serialization/deserialization algorithm should work. You just need to ensure that a binary tree can be serialized to a string and this string can be deserialized to the original tree structure.

// Example:
// You may serialize the following tree:
//     1
//    / \
//   2   3
//      / \
//     4   5
// as "[1,2,3,null,null,4,5]"
// Clarification: The above format is the same as how LeetCode serializes a binary tree. You do not necessarily need to follow this format, so please be creative and come up with different approaches yourself.

// Note: Do not use class member/global/static variables to store states. Your serialize and deserialize algorithms should be stateless.

use crate::base::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Codec;
impl Codec {
    fn serialize(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut data = String::new();
        data.push('[');
        let mut nodes: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        nodes.push_back(root);
        while !nodes.is_empty() {
            if let Some(node) = nodes.pop_front().unwrap() {
                data.push_str(&node.borrow().val.to_string());
                nodes.push_back(node.borrow_mut().left.take());
                nodes.push_back(node.borrow_mut().right.take());
            } else {
                data.push_str("null");
            }
            data.push(',');
        }
        let mut chars: Vec<char> = data.chars().collect();
        for i in (0..chars.len()).rev() {
            if chars[i].is_digit(10) {
                break;
            }
            chars.pop();
        }
        chars.push(']');
        return chars.iter().collect::<String>();
    }

    /**
     * 队列
     * 将节点依次加入队列中，每次遍历队列中的一层树
     */
    fn deserialize(data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let vals: Vec<Option<i32>> = data[1..data.len() - 1]
            .split(",")
            .map(|s| match s.parse::<i32>() {
                Ok(n) => Some(n),
                Err(_) => None,
            })
            .collect();

        let mut nodes: VecDeque<*mut Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap()))));
        nodes.push_back(&mut root);
        let mut index = 1;
        unsafe {
            while index < vals.len() {
                for _ in 0..nodes.len() {
                    if let Some(node) = &*nodes.pop_front().unwrap() {
                        if index < vals.len() {
                            if let Some(val) = vals[index] {
                                node.borrow_mut().left =
                                    Some(Rc::new(RefCell::new(TreeNode::new(val))));
                                nodes.push_back(&mut node.borrow_mut().left);
                            }
                            index += 1;
                        }
                        if index < vals.len() {
                            if let Some(val) = vals[index] {
                                node.borrow_mut().right =
                                    Some(Rc::new(RefCell::new(TreeNode::new(val))));
                                nodes.push_back(&mut node.borrow_mut().right);
                            }
                            index += 1;
                        }
                    }
                }
            }
            return root;
        }
    }
}

#[test]
fn run() {
    let data = "[1,2,3,null,null,4,5]".to_string();
    let root: Option<Rc<RefCell<TreeNode>>> = Codec::deserialize(data.clone());
    assert_eq!(Codec::serialize(root), data);
}
