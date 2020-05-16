// <下一个排列组合>

// Implement next permutation,
// which rearranges numbers into the lexicographically next greater permutation of numbers.

// If such arrangement is not possible,
// it must rearrange it as the lowest possible order (ie, sorted in ascending order).
// The replacement must be in-place and use only constant extra memory.

// Here are some examples.
// Inputs are in the left-hand column and its corresponding outputs are in the right-hand column.
// 1,2,3 → 1,3,2
// 3,2,1 → 1,2,3
// 1,1,5 → 1,5,1

// 解题思路：
// 方法一：
//  

struct Solution;
impl Solution {
    //TODO  It's hard for me.
    pub fn next_permutation(nums: &mut Vec<i32>) {
        
    }
}

#[test]
fn run() {
    // let mut a = vec![3, 2, 1];
    // let mut b = vec![1, 1, 5];
    // let mut c = vec![1, 3, 2];
    // let mut d = vec![1, 2, 3];
    let mut e = vec![2, 3, 1];

    // Solution::next_permutation(&mut a);
    // Solution::next_permutation(&mut b);
    // Solution::next_permutation(&mut c);
    // Solution::next_permutation(&mut d);
    Solution::next_permutation(&mut e);

    // assert_eq!(a, vec![1, 2, 3]);
    // assert_eq!(b, vec![1, 5, 1]);
    // assert_eq!(c, vec![2, 1, 3]);
    // assert_eq!(d, vec![1, 3, 2]);
    assert_eq!(e, vec![3, 1, 2]);
}
