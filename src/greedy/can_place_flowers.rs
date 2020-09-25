// <种花问题>

// Suppose you have a long flowerbed in which some of the plots are planted and some are not. However, flowers cannot be planted in adjacent plots - they would compete for water and both would die.
// Given a flowerbed (represented as an array containing 0 and 1, where 0 means empty and 1 means not empty), and a number n, return if n new flowers can be planted in it without violating the no-adjacent-flowers rule.

// Example 1:
// Input: flowerbed = [1,0,0,0,1], n = 1
// Output: True

// Example 2:
// Input: flowerbed = [1,0,0,0,1], n = 2
// Output: False

// Note:
// The input array won't violate no-adjacent-flowers rule.
// The input array size is in the range of [1, 20000].
// n is a non-negative integer which won't exceed the input array size.

struct Solution;
impl Solution {
    /**
     * 贪心算法
     * 从左往右遍历，依次将中间值遍历到合法的值改为1
     */
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut flowerbed = flowerbed;
        let mut n = n;
        let mut i = 0;
        while i < flowerbed.len() {
            // NOTE 如果为左右边界则设置为0
            let pre = if i > 0 { flowerbed[i - 1] } else { 0 };
            let next = if i + 1 == flowerbed.len() {
                0
            } else {
                flowerbed[i + 1]
            };
            // NOTE 取左中右三个值，必须都为0才可以设置中间值为1
            if pre == 0 && flowerbed[i] == 0 && next == 0 {
                flowerbed[i] = 1;
                n -= 1;
            }
            i += 1;
        }
        return n <= 0;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
    assert_eq!(
        Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2),
        false
    );
}
