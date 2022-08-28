// <单词替换>

// In English, we have a concept called root, which can be followed by some other word to form another longer word - let's call this word successor. For example, when the root "an" is followed by the successor word "other", we can form a new word "another".
// Given a dictionary consisting of many roots and a sentence consisting of words separated by spaces, replace all the successors in the sentence with the root forming it. If a successor can be replaced by more than one root, replace it with the root that has the shortest length.
// Return the sentence after the replacement.

// Example 1:
// Input: dictionary = ["cat","bat","rat"], sentence = "the cattle was rattled by the battery"
// Output: "the cat was rat by the bat"

// Example 2:
// Input: dictionary = ["a","b","c"], sentence = "aadsfasf absbs bbab cadsfafs"
// Output: "a a b c"

// Constraints:
// 1 <= dictionary.length <= 1000
// 1 <= dictionary[i].length <= 100
// dictionary[i] consists of only lower-case letters.
// 1 <= sentence.length <= 106
// sentence consists of only lower-case letters and spaces.
// The number of words in sentence is in the range [1, 1000]
// The length of each word in sentence is in the range [1, 1000]
// Every two consecutive words in sentence will be separated by exactly one space.
// sentence does not have leading or trailing spaces.

use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let dicts = dictionary.into_iter().fold(HashSet::new(), |mut s, w| {
            s.insert(w);
            return s;
        });
        let words: Vec<&str> = sentence.split(' ').collect();
        let mut res: Vec<&str> = vec![];
        'outer: for word in words.into_iter() {
            for i in 1..word.len() {
                if dicts.contains(&word[..i]) {
                    res.push(&word[..i]);
                    continue 'outer;
                }
            }
            res.push(word);
        }
        return res.join(" ");
    }
}
