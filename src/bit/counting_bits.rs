// <比特位计数>

// Given a non negative integer number num.
// For every numbers i in the range 0 ≤ i ≤ num calculate the number of 1's in their binary representation and return them as an array.

// Example 1:
// Input: 2
// Output: [0,1,1]

// Example 2:
// Input: 5
// Output: [0,1,1,2,1,2]

// Follow up:
// It is very easy to come up with a solution with run time O(n*sizeof(integer)). But can you do it in linear time O(n) /possibly in a single pass?
// Space complexity should be O(n).
// Can you do it like a boss? Do it without using any builtin function like __builtin_popcount in c++ or in any other language.

struct Solution;
/*
 * 0  1  2  3  4  5  6  7  8  9
 * 0  1  1  2  1  2  2  3  1  2
 */
impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; num as usize + 1];
        for n in 1..=num as usize {
            res[n] = res[n >> 1] + (n & 1) as i32;
        }
        return res;
    }

    /**
     * 动态规划
     * 当数字等于2的幂时，总为1
     */
    pub fn count_bits2(num: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; num as usize + 1];
        let mut div = 1;
        for n in 1..=num as usize {
            let tmp = n % div;
            if tmp == 0 {
                res[n] = 1;
                div = n;
            } else {
                res[n] = res[tmp] + 1;
            }
        }
        return res;
    }

    /*
     * 暴力法，利用rust函数
     */
    pub fn count_bits3(num: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        for n in 0..=num {
            res.push(n.count_ones() as i32);
        }
        return res;
    }

    /**
     * 暴力法，遍历每个数字
     */
    pub fn count_bits4(num: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        for mut n in 0..=num {
            let mut count = 0;
            while n != 0 {
                if n & 1 == 1 {
                    count += 1;
                }
                n >>= 1;
            }
            res.push(count);
        }
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
    assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
}
