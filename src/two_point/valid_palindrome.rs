// <验证回文字符串>

// Given a string, determine if it is a palindrome, considering only alphanumeric characters and ignoring cases.
// Note: For the purpose of this problem, we define empty string as valid palindrome.

// Example 1:
// Input: "A man, a plan, a canal: Panama"
// Output: true

// Example 2:
// Input: "race a car"
// Output: false

struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.to_lowercase().chars().collect();
        let mut l: i32 = 0;
        let mut r: i32 = s.len() as i32 - 1;
        while l <= r {
            let lc = chars[l as usize];
            let rc = chars[r as usize];
            if lc.is_alphanumeric() && rc.is_alphanumeric() {
                if lc != rc {
                    return false;
                } else {
                    l += 1;
                    r -= 1;
                }
            } else if lc.is_alphanumeric() {
                r -= 1;
            } else {
                l += 1;
            }
        }
        return true;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::is_palindrome("8V8K;G;K;V;".to_string()), false);
    assert_eq!(
        Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
        true
    );
    assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
}
