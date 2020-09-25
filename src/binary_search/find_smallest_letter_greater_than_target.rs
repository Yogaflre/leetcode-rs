// <寻找比目标字母大的最小字母>

// Given a list of sorted characters letters containing only lowercase letters, and given a target letter target, find the smallest element in the list that is larger than the given target.
// Letters also wrap around. For example, if the target is target = 'z' and letters = ['a', 'b'], the answer is 'a'.

// Examples:
// Input:
// letters = ["c", "f", "j"]
// target = "a"
// Output: "c"

// Input:
// letters = ["c", "f", "j"]
// target = "c"
// Output: "f"

// Input:
// letters = ["c", "f", "j"]
// target = "d"
// Output: "f"

// Input:
// letters = ["c", "f", "j"]
// target = "g"
// Output: "j"

// Input:
// letters = ["c", "f", "j"]
// target = "j"
// Output: "c"

// Input:
// letters = ["c", "f", "j"]
// target = "k"
// Output: "c"

// Note:
// letters has a length in range [2, 10000].
// letters consists of lowercase letters, and contains at least 2 unique letters.
// target is a lowercase letter.

struct Solution;
impl Solution {
    /**
     * 变种二分查找，利用字符ASCII码的特性
     */
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut letters = letters;
        // 如果是有序的情况就不需要排序了
        letters.sort();
        let mut l: i32 = 0;
        let mut r: i32 = letters.len() as i32 - 1;
        while l <= r {
            let mid = (l + r) / 2;
            // NOTE 题目要求找大于target的值，所以小于等于直接略过
            if letters[mid as usize] <= target {
                l = mid + 1;
            } else {
                // NOTE 当前mid大于target，如果前一个字符<=target，那说明当前是第一个大于target的字符
                if mid >= 1 && letters[mid as usize - 1] <= target {
                    return letters[mid as usize];
                } else {
                    r = mid - 1;
                }
            }
        }
        // NOTE 当没找到时直接返回第一个元素
        return letters[0];
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'c'),
        'f'
    );
}
