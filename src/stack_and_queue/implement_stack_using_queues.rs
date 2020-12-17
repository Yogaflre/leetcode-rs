// <用队列实现栈>

// Implement a last in first out (LIFO) stack using only two queues. The implemented stack should support all the functions of a normal queue (push, top, pop, and empty).
// Implement the MyStack class:
// void push(int x) Pushes element x to the top of the stack.
// int pop() Removes the element on the top of the stack and returns it.
// int top() Returns the element on the top of the stack.
// boolean empty() Returns true if the stack is empty, false otherwise.

// Notes:
// You must use only standard operations of a queue, which means only push to back, peek/pop from front, size, and is empty operations are valid.
// Depending on your language, the queue may not be supported natively. You may simulate a queue using a list or deque (double-ended queue), as long as you use only a queue's standard operations.
// Follow-up: Can you implement the stack such that each operation is amortized O(1) time complexity? In other words, performing n operations will take overall O(n) time even if one of those operations may take longer.

// Example 1:
// Input
// ["MyStack", "push", "push", "top", "pop", "empty"]
// [[], [1], [2], [], [], []]
// Output
// [null, null, null, 2, 2, false]
// Explanation
// MyStack myStack = new MyStack();
// myStack.push(1);
// myStack.push(2);
// myStack.top(); // return 2
// myStack.pop(); // return 2
// myStack.empty(); // return False

// Constraints:
// 1 <= x <= 9
// At most 100 calls will be made to push, pop, top, and empty.
// All the calls to pop and top are valid.

use std::collections::VecDeque;
// master_queue作为主要操作队列，assist_queue只用来辅助master_queue构成stack
struct MyStack {
    master_queue: VecDeque<i32>,
    assist_queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyStack {
            master_queue: VecDeque::new(),
            assist_queue: VecDeque::new(),
        }
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        // 1.push到辅助队列
        self.assist_queue.push_back(x);
        // 2.将主队列全部拼接到辅助队列，每次得到的都符合stack特性
        self.assist_queue.append(&mut self.master_queue);
        // 3.交换主队列和辅助队列的指针
        std::mem::swap(&mut self.master_queue, &mut self.assist_queue);
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        return self.master_queue.pop_front().unwrap();
    }

    /** Get the top element. */
    fn top(&self) -> i32 {
        return self.master_queue[0];
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        return self.master_queue.is_empty();
    }
}

#[test]
fn run() {
    let mut stack = MyStack::new();
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.top(), 2);
    assert_eq!(stack.pop(), 2);
    assert_eq!(stack.empty(), false);
}
