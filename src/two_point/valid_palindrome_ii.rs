// <验证回文字符串 Ⅱ>

// Given a non-empty string s, you may delete at most one character. Judge whether you can make it a palindrome.
// Example 1:
// Input: "aba"
// Output: True

// Example 2:
// Input: "abca"
// Output: True
// Explanation: You could delete the character 'c'.
// Note:
// The string will only contain lowercase characters a-z. The maximum length of the string is 50000.

struct Solution;
impl Solution {
    /**
     * 双指针 + 递归
     * 依次比较左右元素是否相等。对可以删除一个字符进行容错
     * 注意：只能删除一个字符，删除l还是r的字符需要分两种情况判断
     */
    pub fn valid_palindrome(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        // NOTE 因为只有一个字符可以被删除，所以必须加一个标记为来判断是否已经执行过删除操作
        return Self::recursive(&s, 0, s.len() - 1, true);
    }

    fn recursive(s: &Vec<char>, l: usize, r: usize, flag: bool) -> bool {
        if l > r {
            return true;
        }
        if s[l] != s[r] {
            if flag {
                // NOTE 因为不确定哪个索引位置的字符可以删除，所以需要递归两种情况
                return Self::recursive(s, l + 1, r, false) || Self::recursive(s, l, r - 1, false);
            } else {
                return false;
            }
        } else {
            return Self::recursive(s, l + 1, r - 1, flag);
        }
    }
}

#[test]
fn run() {
    assert_eq!(Solution::valid_palindrome("aba".to_string()), true);
    assert_eq!(Solution::valid_palindrome("abca".to_string()), true);
    assert_eq!(
        Solution::valid_palindrome("eeccccbebaeeabebccceea".to_string()),
        false
    );
    assert_eq!(
        Solution::valid_palindrome("aguokepatgbnvfqmgmlcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupuculmgmqfvnbgtapekouga".to_string()),
        true
    );
}
