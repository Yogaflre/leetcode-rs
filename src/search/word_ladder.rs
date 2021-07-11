// <单词接龙>

// A transformation sequence from word beginWord to word endWord using a dictionary wordList is a sequence of words beginWord -> s1 -> s2 -> ... -> sk such that:
// Every adjacent pair of words differs by a single letter.
// Every si for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
// sk == endWord
// Given two words, beginWord and endWord, and a dictionary wordList, return the number of words in the shortest transformation sequence from beginWord to endWord, or 0 if no such sequence exists.

// Example 1:
// Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
// Output: 5
// Explanation: One shortest transformation sequence is "hit" -> "hot" -> "dot" -> "dog" -> cog", which is 5 words long.

// Example 2:
// Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
// Output: 0
// Explanation: The endWord "cog" is not in wordList, therefore there is no valid transformation sequence.

// Constraints:
// 1 <= beginWord.length <= 10
// endWord.length == beginWord.length
// 1 <= wordList.length <= 5000
// wordList[i].length == beginWord.length
// beginWord, endWord, and wordList[i] consist of lowercase English letters.
// beginWord != endWord
// All the words in wordList are unique.

use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut word_pairs: Vec<(&String, bool)> = word_list.iter().map(|s| (s, false)).collect();
        let mut dp: HashMap<String, i32> = HashMap::new();
        let res = Self::dfs(&begin_word, &end_word, &mut word_pairs, &mut dp);
        return res;
    }

    fn dfs(
        begin_word: &String,
        end_word: &String,
        word_pairs: &mut Vec<(&String, bool)>,
        dp: &mut HashMap<String, i32>,
    ) -> i32 {
        if begin_word.eq(end_word) {
            return 1;
        }
        if let Some(n) = dp.get(begin_word) {
            return *n;
        }
        let mut res: i32 = i32::max_value();
        for i in 0..word_pairs.len() {
            if !word_pairs[i].1 && Self::check_single_diff(begin_word, &word_pairs[i].0) {
                word_pairs[i].1 = true;
                let tmp = Self::dfs(&word_pairs[i].0, end_word, word_pairs, dp);
                if tmp != 0 && tmp < res {
                    res = tmp;
                }
                word_pairs[i].1 = false;
            }
        }
        if res == i32::max_value() {
            res = 0;
        } else {
            res += 1;
        }
        dp.insert(begin_word.into(), res);
        return res;
    }

    fn check_single_diff(word1: &str, word2: &str) -> bool {
        let mut diff = 0;
        let mut w1 = word1.chars();
        let mut w2 = word2.chars();
        while let (Some(w1), Some(w2)) = (w1.next(), w2.next()) {
            if w1 != w2 {
                diff += 1;
            }
        }
        return diff == 1;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::ladder_length(
            "hig".to_string(),
            "cig".to_string(),
            vec!["cig".to_string()]
        ),
        2
    );

    assert_eq!(
        Solution::ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            vec![
                "hot".to_string(),
                "dot".to_string(),
                "dog".to_string(),
                "lot".to_string(),
                "log".to_string()
            ]
        ),
        0
    );

    assert_eq!(
        Solution::ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            vec![
                "hot".to_string(),
                "dot".to_string(),
                "dog".to_string(),
                "lot".to_string(),
                "log".to_string(),
                "cog".to_string()
            ]
        ),
        5
    );
}
