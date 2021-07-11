// <翻转整型>
// Given a 32-bit signed integer, reverse digits of an integer.

// Example 1:
// Input: 123
// Output: 321

// Example 2:
// Input: -123
// Output: -321

// Example 3:
// Input: 120
// Output: 21

// Note:
// Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.
// Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

struct Solution;
impl Solution {
    /*
     * 取余 + 位溢出
     */
    pub fn reverse(mut x: i32) -> i32 {
        let check_multi = |tmp| -> bool {
            if tmp > 0 {
                return i32::MAX / 10 >= tmp;
            } else {
                return i32::MIN / 10 <= tmp;
            }
        };
        let check_add = |tmp: i32, add: i32| {
            if tmp > 0 {
                return i32::MAX - tmp >= add;
            } else {
                return i32::MIN - tmp <= add;
            }
        };

        let mut res = 0;
        while x != 0 {
            if check_multi(res) {
                res *= 10;
            } else {
                return 0;
            }
            let add = x % 10;
            if check_add(res, add) {
                res += add;
                x /= 10;
            } else {
                return 0;
            }
        }
        return res;
    }
    fn check_overflow(tmp: i32, res: i32, add: i32) -> bool {
        if tmp > 0 {
            if i32::MAX / 10 > tmp / 10 {
                return (tmp / 10 - add) == res;
            } else {
                return false;
            }
        } else {
            if i32::MIN / 10 < tmp / 10 {
                return (tmp / 10 - add) == res;
            } else {
                return false;
            }
        }
    }

    /*
     * 将input转换为i64的绝对值，并记录正负、将input:i64转换为Vec<char>、对每个数字乘以对应位数构造最终结果、对最终值赋予正负
     */
    pub fn reverse_by_string(x: i32) -> i32 {
        let flag: bool = x > 0;
        let x: i64 = (x as i64).abs();
        let mut temp: i64 = 1;
        let mut result: i64 = 0;
        for i in x.to_string().chars() {
            result += i.to_digit(10).unwrap() as i64 * temp;
            temp *= 10;
        }
        result = if flag { result } else { result * -1 };
        return if result > i32::MAX as i64 || result < i32::MIN as i64 {
            0
        } else {
            result as i32
        };
    }
}

#[test]
pub fn run() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(-2147483648), 0);
}
