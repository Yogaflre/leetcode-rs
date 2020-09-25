// <根据字符出现频率排序>

// Given a string, sort it in decreasing order based on the frequency of characters.

// Example 1:
// Input:
// "tree"
// Output:
// "eert"
// Explanation:
// 'e' appears twice while 'r' and 't' both appear once.
// So 'e' must appear before both 'r' and 't'. Therefore "eetr" is also a valid answer.

// Example 2:
// Input:
// "cccaaa"
// Output:
// "cccaaa"
// Explanation:
// Both 'c' and 'a' appear three times, so "aaaccc" is also a valid answer.
// Note that "cacaca" is incorrect, as the same characters must be together.

// Example 3:
// Input:
// "Aabb"
// Output:
// "bbAa"
// Explanation:
// "bbaA" is also a valid answer, but "Aabb" is incorrect.
// Note that 'A' and 'a' are treated as two different characters.

use std::collections::{BinaryHeap, HashMap};
struct Solution;
impl Solution {
    /**
     * 堆排序
     */
    pub fn frequency_sort(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut map: HashMap<char, usize> = HashMap::new();
        for c in s {
            if let Some(v) = map.get_mut(&c) {
                *v += 1;
            } else {
                map.insert(c, 1);
            }
        }
        // NOTE 构造出现频率和字符的大顶堆，依次取频率最高的字符添加到结果集中
        let mut heap: BinaryHeap<(usize, char)> = BinaryHeap::new();
        for entry in map {
            heap.push((entry.1, entry.0));
        }
        let mut res = String::new();
        while let Some((f, c)) = heap.pop() {
            for _ in 0..f {
                res.push(c);
            }
        }
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::frequency_sort("tree".to_string()),
        "eetr".to_string()
    );
    assert_eq!(
        Solution::frequency_sort("cccaaa".to_string()),
        "cccaaa".to_string()
    );
    assert_eq!(
        Solution::frequency_sort("Aabb".to_string()),
        "bbaA".to_string()
    );
}
