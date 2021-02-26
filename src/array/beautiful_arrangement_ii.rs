// <优美的排列 II>

// Given two integers n and k, you need to construct a list which contains n different positive integers ranging from 1 to n and obeys the following requirement:
// Suppose this list is [a1, a2, a3, ... , an], then the list [|a1 - a2|, |a2 - a3|, |a3 - a4|, ... , |an-1 - an|] has exactly k distinct integers.
// If there are multiple answers, print any of them.

// Example 1:
// Input: n = 3, k = 1
// Output: [1, 2, 3]
// Explanation: The [1, 2, 3] has three different positive integers ranging from 1 to 3, and the [1, 1] has exactly 1 distinct integer: 1.

// Example 2:
// Input: n = 3, k = 2
// Output: [1, 3, 2]
// Explanation: The [1, 3, 2] has three different positive integers ranging from 1 to 3, and the [2, 1] has exactly 2 distinct integers: 1 and 2.

// Note:
// The n and k are in the range 1 <= k < n <= 104.

struct Solution;
impl Solution {
    /*
     * 反转数组
     * 1 2 3 4 5 6
     * k=1: 1 6 5 4 3 2 (反转1次)
     * k=2: 1 6 2 3 4 5 (反转2次)
     * k=3: 1 6 2 5 4 3 (反转3次)
     * ...
     */
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut lower = 1;
        let mut upper = n;
        let mut reverse = false;
        let mut k = k;
        while lower <= upper {
            if !reverse {
                res.push(lower);
                lower += 1;
            } else {
                res.push(upper);
                upper -= 1;
            }
            k -= 1;
            if k > 0 {
                reverse = !reverse;
            }
        }
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::construct_array(3, 1), vec![1, 2, 3]);
    assert_eq!(Solution::construct_array(3, 2), vec![1, 3, 2]);
}
