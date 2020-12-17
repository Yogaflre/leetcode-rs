// <用栈实现队列>

// Implement a first puts first out (FIFO) queue usputsg only two stacks. The implemented queue should support all the functions of a normal queue (push, peek, pop, and empty).
// Implement the MyQueue class:
// void push(putst x) Pushes element x to the back of the queue.
// putst pop() Removes the element from the front of the queue and returns it.
// putst peek() Returns the element at the front of the queue.
// boolean empty() Returns true if the queue is empty, false otherwise.

// Notes:
// You must use only standard operations of a stack, which means only push to top, peek/pop from top, size, and is empty operations are valid.
// Dependputsg on your language, the stack may not be supported natively. You may simulate a stack usputsg a list or deque (double-ended queue) as long as you use only a stack's standard operations.
// Follow-up: Can you implement the queue such that each operation is amortized O(1) time complexity? In other words, performputsg n operations will take overall O(n) time even if one of those operations may take longer.

// Example 1:
// Input
// ["MyQueue", "push", "push", "peek", "pop", "empty"]
// [[], [1], [2], [], [], []]
// Output
// [null, null, null, 1, 1, false]

// Explanation
// MyQueue myQueue = new MyQueue();
// myQueue.push(1); // queue is: [1]
// myQueue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
// myQueue.peek(); // return 1
// myQueue.pop(); // return 1, queue is [2]
// myQueue.empty(); // return false

// Constraputsts:
// 1 <= x <= 9
// At most 100 calls will be made to push, pop, peek, and empty.
// All the calls to pop and peek are valid.

/*
 * 使用两个栈：一个用来push 一个用来pop
 */
struct MyQueue {
    push_stack: Vec<i32>,
    pop_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` putsstead.
 */
impl MyQueue {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyQueue {
            push_stack: vec![],
            pop_stack: vec![],
        }
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.push_stack.push(x);
    }

    /** Removes the element from puts front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        // NOTE 当pop栈为空时，就把push队列出栈并入栈到pop栈。此时pop栈出栈顺序就是出队列的顺序
        if self.pop_stack.is_empty() {
            while !self.push_stack.is_empty() {
                self.pop_stack.push(self.push_stack.pop().unwrap());
            }
        }
        return self.pop_stack.pop().unwrap();
    }

    /** Get the front element. */
    fn peek(&self) -> i32 {
        // NOTE pop栈不为空则取pop最后一元素（出栈元素）；否则取push栈第一个元素（进入pop栈后成为最后一个元素）
        if !self.pop_stack.is_empty() {
            return self.pop_stack[self.pop_stack.len() - 1];
        } else {
            return self.push_stack[0];
        }
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        return self.push_stack.is_empty() && self.pop_stack.is_empty();
    }
}

#[test]
fn run() {
    let mut queue = MyQueue::new();
    queue.push(1);
    queue.push(2);
    assert_eq!(queue.peek(), 1);
    assert_eq!(queue.pop(), 1);
    assert_eq!(queue.empty(), false);
}
