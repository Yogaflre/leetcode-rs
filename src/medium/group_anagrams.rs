// <变位词组>
// Given an array of strings, group anagrams together.

// Example:
// Input: ["eat", "tea", "tan", "ate", "nat", "bat"],

// Output:
// [
//   ["ate","eat","tea"],
//   ["nat","tan"],
//   ["bat"]
// ]

// Note:
// All inputs will be in lowercase.
// The order of your output does not matter.

use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();
        for s in strs {
            let mut cs: Vec<char> = s.chars().collect();
            cs.sort();
            if let Some(v) = map.get_mut(&cs) {
                v.push(s);
            } else {
                map.insert(cs, vec![s]);
            }
        }
        return map.values().map(|x| x.to_vec()).collect();
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string()
        ]),
        vec![
            vec!["nat".to_string(), "tan".to_string()],
            vec!["bat".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()]
        ]
    );
}
