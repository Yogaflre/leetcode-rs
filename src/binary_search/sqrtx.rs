// <x的平方根>

// Implement int sqrt(int x).
// Compute and return the square root of x, where x is guaranteed to be a non-negative integer.
// Since the return type is an integer, the decimal digits are truncated and only the integer part of the result is returned.

// Example 1:
// Input: 4
// Output: 2

// Example 2:
// Input: 8
// Output: 2
// Explanation: The square root of 8 is 2.82842..., and since
//              the decimal part is truncated, 2 is returned.

struct Solution;
impl Solution {
    /**
     * 二分查找
     * 每次左右边界和中点的值，判断平方根，不断缩小区间
     */
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let mut l = 1;
        let mut r = x;
        loop {
            let mid = (l + r) / 2;
            if mid > x / mid {
                r = mid - 1;
            } else {
                // NOTE 因为只取整数，需要判断 mid~mid+1之间是否存在合法值
                if mid + 1 > x / (mid + 1) {
                    return mid;
                } else {
                    l = mid + 1;
                }
            }
        }
    }
}

#[test]
fn run() {
    assert_eq!(Solution::my_sqrt(4), 2);
    assert_eq!(Solution::my_sqrt(8), 2);
}
