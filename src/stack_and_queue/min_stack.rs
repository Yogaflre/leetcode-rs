// <最小栈>

// Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.

// push(x) -- Push element x onto stack.
// pop() -- Removes the element on top of the stack.
// top() -- Get the top element.
// getMin() -- Retrieve the minimum element in the stack.

// Example 1:
// Input
// ["MinStack","push","push","push","getMin","pop","top","getMin"]
// [[],[-2],[0],[-3],[],[],[],[]]

// Output
// [null,null,null,null,-3,null,0,-2]

// Explanation
// MinStack minStack = new MinStack();
// minStack.push(-2);
// minStack.push(0);
// minStack.push(-3);
// minStack.getMin(); // return -3
// minStack.pop();
// minStack.top();    // return 0
// minStack.getMin(); // return -2

struct MinStack {
    stack: Vec<i32>,
    minimum: Option<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            stack: vec![],
            minimum: None,
        }
    }

    fn push(&mut self, x: i32) {
        if let Some(m) = self.minimum {
            if x < m {
                self.minimum = Some(x);
            }
        }
        self.stack.push(x);
    }

    fn pop(&mut self) {
        let del = self.stack.pop();
        if del == self.minimum {
            self.minimum = None;
        }
    }

    fn top(&self) -> i32 {
        return self.stack[self.stack.len() - 1];
    }

    fn get_min(&mut self) -> i32 {
        return self.minimum.unwrap_or_else(|| {
            let m: i32 = *(self.stack.iter().min().unwrap());
            self.minimum = Some(m);
            return m;
        });
    }
}

#[test]
fn run() {
    let mut stack = MinStack::new();
    stack.push(-2);
    stack.push(0);
    stack.push(-3);
    assert_eq!(stack.get_min(), -3);
    stack.pop();
    assert_eq!(stack.top(), 0);
    assert_eq!(stack.get_min(), -2);
}
