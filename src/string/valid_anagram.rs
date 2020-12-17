// <有效的字母异位词>

// Given two strings s and t , write a function to determine if t is an anagram of s.

// Example 1:
// Input: s = "anagram", t = "nagaram"
// Output: true

// Example 2:
// Input: s = "rat", t = "car"
// Output: false

// Note:
// You may assume the string contains only lowercase alphabets.

// Follow up:
// What if the inputs contain unicode characters? How would you adapt your solution to such case?

struct Solution;
impl Solution {
    /*
     * 三次遍历
     * 第一次记录s的所有字符count值
     * 第二次减去t的所有字符count值
     * 第三次遍历counts列表，查看是否有不为0的值。不为0说明两个字符个数不想等
     */
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut counts: Vec<i32> = vec![0; 26];
        for c in s.chars() {
            counts[((c as u32) - ('a' as u32)) as usize] += 1;
        }
        for c in t.chars() {
            counts[((c as u32) - ('a' as u32)) as usize] -= 1;
        }
        for count in counts {
            if count != 0 {
                return false;
            }
        }
        return true;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::is_anagram(String::from("anagram"), String::from("nagaram")),
        true
    );
    assert_eq!(
        Solution::is_anagram(String::from("rat"), String::from("car")),
        false
    );
}
