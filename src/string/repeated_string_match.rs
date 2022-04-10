// <重复叠加字符串匹配>
// Given two strings a and b, return the minimum number of times you should repeat string a so that string b is a substring of it. If it is impossible for b​​​​​​ to be a substring of a after repeating it, return -1.
// Notice: string "abc" repeated 0 times is "", repeated 1 time is "abc" and repeated 2 times is "abcabc".

// Example 1:
// Input: a = "abcd", b = "cdabcdab"
// Output: 3
// Explanation: We return 3 because by repeating a three times "abcdabcdabcd", b is a substring of it.

// Example 2:
// Input: a = "a", b = "aa"
// Output: 2

// Constraints:

//     1 <= a.length, b.length <= 104
//     a and b consist of lowercase English letters.

struct Solution;
impl Solution {
    /*
     * 将a添加到足够包含b的长度，判断是否包含b
     */
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        if b.is_empty() {
            return 0;
        }
        let mut count = 1;
        let mut multi_a = String::from(&a);
        while multi_a.len() < b.len() {
            multi_a.push_str(&a);
            count += 1;
        }
        if multi_a.contains(&b) {
            return count;
        }
        // NOTE 当a长度 > b长度时，检验两个a拼接起来是否包含b（e.g.: aaab, ba）
        multi_a.push_str(&a);
        if multi_a.contains(&b) {
            return count + 1;
        }
        return -1;
    }
}
