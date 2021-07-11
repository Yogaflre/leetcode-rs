// <编辑距离(莱文斯坦距离)>

// Given two words word1 and word2, find the minimum number of operations required to convert word1 to word2.
// You have the following 3 operations permitted on a word:
// Insert a character
// Delete a character
// Replace a character

// Example 1:
// Input: word1 = "horse", word2 = "ros"
// Output: 3
// Explanation:
// horse -> rorse (replace 'h' with 'r')
// rorse -> rose (remove 'r')
// rose -> ros (remove 'e')

// Example 2:
// Input: word1 = "intention", word2 = "execution"
// Output: 5
// Explanation:
// intention -> inention (remove 't')
// inention -> enention (replace 'i' with 'e')
// enention -> exention (replace 'n' with 'x')
// exention -> exection (replace 'n' with 'c')
// exection -> execution (insert 'u')

use std::cmp::min;
struct Solution;
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut matrix: Vec<Vec<i32>> = vec![vec![-1; word2.len()]; word1.len()];
        return Self::distance(
            &word1.chars().collect(),
            &word2.chars().collect(),
            0,
            0,
            &mut matrix,
        );
    }

    fn distance(
        word1: &Vec<char>,
        word2: &Vec<char>,
        i: usize,
        j: usize,
        matrix: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if i < word1.len() && j < word2.len() {
            if matrix[i][j] != -1 {
                return matrix[i][j];
            }
        }
        let tmp: i32;
        if i == word1.len() || j == word2.len() {
            if i < word1.len() {
                tmp = (word1.len() - i) as i32;
            } else if j < word2.len() {
                tmp = (word2.len() - j) as i32;
            } else {
                tmp = 0;
            }
        } else {
            if word1[i] == word2[j] {
                tmp = Self::distance(word1, word2, i + 1, j + 1, matrix);
            } else {
                let tmp1 = Self::distance(word1, word2, i + 1, j + 1, matrix);
                let tmp2 = Self::distance(word1, word2, i, j + 1, matrix);
                let tmp3 = Self::distance(word1, word2, i + 1, j, matrix);
                tmp = 1 + min(tmp1, min(tmp2, tmp3));
            }
            matrix[i][j] = tmp;
        }
        return tmp;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::min_distance("horse".to_string(), "ros".to_string()),
        3
    );
    assert_eq!(
        Solution::min_distance("intention".to_string(), "execution".to_string()),
        5
    );
}
