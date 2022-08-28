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
        for s in strs.into_iter() {
            let mut cs: Vec<char> = s.chars().collect();
            cs.sort();
            if let Some(tmp) = map.get_mut(&cs) {
                tmp.push(s);
            } else {
                map.insert(cs, vec![s]);
            }
        }
        return map.values().map(|v| v.to_vec()).collect();
    }
}
