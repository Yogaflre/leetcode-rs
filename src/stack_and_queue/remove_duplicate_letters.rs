// <去除重复字母>
// Given a string s, remove duplicate letters so that every letter appears once and only once. You must make sure your result is the smallest in lexicographical order among all possible results.

// Example 1:
// Input: s = "bcabc"
// Output: "abc"

// Example 2:
// Input: s = "cbacdcbc"
// Output: "acdb"

// Constraints:
//     1 <= s.length <= 104
//     s consists of lowercase English letters.

struct Solution;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut count: Vec<usize> = vec![0; 26]; // 保存每个字符的个数
        let mut exist: Vec<bool> = vec![false; 26]; // 保存该字符是否被使用
        for c in s.chars() {
            count[c as usize - 'a' as usize] += 1;
        }
        let mut cs: Vec<char> = vec![]; // 使用栈保存当前字符，栈中的每次状态均必须保证合法性
        for c in s.chars() {
            let c_index = c as usize - 'a' as usize;
            if exist[c_index] == true {
                count[c_index] -= 1;
                continue;
            }
            while let Some(pre) = cs.last() {
                let p_index = *pre as usize - 'a' as usize;
                if count[p_index] > 0 && *pre > c {
                    exist[p_index] = false;
                    cs.pop().unwrap();
                } else {
                    break;
                }
            }
            cs.push(c);
            exist[c_index] = true;
            count[c_index] -= 1;
        }
        return cs.into_iter().collect();
    }
}
