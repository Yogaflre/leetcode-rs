// <最长公共前缀>

// Write a function to find the longest common prefix string amongst an array of strings.
// If there is no common prefix, return an empty string "".

// Example 1:
// Input: ["flower","flow","flight"]
// Output: "fl"

// Example 2:
// Input: ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.

// Note: All given inputs are in lowercase letters a-z.

struct Solution;
impl Solution {
    /**
     * 高科技写法
     */
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut strs = strs.iter();
        if let Some(head) = strs.next().cloned() {
            strs.fold(head, |head, tail| {
                head.chars()
                    .zip(tail.chars())
                    .take_while(|(l, r)| l == r)
                    .map(|(chr, _)| chr)
                    .collect()
            })
        } else {
            "".into()
        }
    }

    /**
     * 遍历所有字符串，依次按顺序比较字符，找到最大的重复字符
     */
    pub fn longest_common_prefix2(strs: Vec<String>) -> String {
        let mut common_prefix = String::new();
        if strs.is_empty() {
            return common_prefix;
        }
        let mut indexs: Vec<usize> = vec![0; strs.len()];
        'outer: loop {
            let mut c: char = '0';
            for (i, index) in indexs.iter_mut().enumerate() {
                let tmp_str = &strs[i];
                if *index < tmp_str.len() {
                    let tmp_c = tmp_str.chars().nth(*index).unwrap();
                    if c == '0' {
                        c = tmp_c;
                    } else if c != tmp_c {
                        break 'outer;
                    }
                    *index += 1;
                } else {
                    break 'outer;
                }
            }
            common_prefix.push(c);
        }
        return common_prefix;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ]),
        "fl".to_string()
    );
    assert_eq!(
        Solution::longest_common_prefix(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string(),
        ]),
        "".to_string()
    );
}
