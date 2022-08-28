// <斐波那切数列>

// 请你输出斐波那契数列的第n项（从0开始，第0项为0）。

// 解题思路
// 方法一：
//  当n=1或2时，特征值按照特定的返回值返回。
//  当n>2时，从第三位开始遍历，对最终值求和，并且替换前置和后置的值。
// 方法二：
//  为什么循环比递归快？因为画出递归树就可以看出递归存在严重的重复计算

struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n <= 1 {
            return n;
        } else {
            Self::fib(n - 2) + Self::fib(n - 1)
        }
    }

    pub fn fib_loop(n: i32) -> i32 {
        let mut a = 0;
        let mut b = 1;
        for _ in 0..n {
            let t = b;
            b = a + b;
            a = t;
        }
        a
    }
}

#[test]
pub fn run() {
    assert_eq!(Solution::fib(5), 5);
    assert_eq!(Solution::fib_loop(2), 1);
}
