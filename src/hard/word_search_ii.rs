// <单词搜索2>

// Given a 2D board and a list of words from the dictionary, find all words in the board.
// Each word must be constructed from letters of sequentially adjacent cell, where "adjacent" cells are those horizontally or vertically neighboring. The same letter cell may not be used more than once in a word.

// Example:
// Input:
// board = [
//   ['o','a','a','n'],
//   ['e','t','a','e'],
//   ['i','h','k','r'],
//   ['i','f','l','v']
// ]
// words = ["oath","pea","eat","rain"]
// Output: ["eat","oath"]

// Note:
// All inputs are consist of lowercase letters a-z.
// The values of words are distinct.

use crate::base::trie::Trie;
use std::collections::HashSet;
use std::iter::FromIterator;
struct Solution;
impl Solution {
    /**
     * BF(暴力搜索) + Trie(字典树)
     * 构造字典树(用于快速校验字符是否符合要求)
     * 遍历所有字符，以每个字符为开头进行递归查找(递归时需要用visited记录已访问过得节点)
     */
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut exists: HashSet<String> = HashSet::new();
        let mut trie: Trie = Trie::create(words.iter().map(|w| &w[..]).collect::<Vec<&str>>());

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let mut visited: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];
                Self::search(
                    &board,
                    i,
                    j,
                    String::new(),
                    &mut trie,
                    &mut visited,
                    &mut exists,
                );
            }
        }
        return Vec::from_iter(exists.into_iter());
    }

    fn search(
        board: &Vec<Vec<char>>,
        i: usize,
        j: usize,
        mut tmp: String,
        trie: &mut Trie,
        visited: &mut Vec<Vec<bool>>,
        exists: &mut HashSet<String>,
    ) {
        if visited[i][j] {
            return;
        }
        tmp.push(board[i][j]);
        visited[i][j] = true;
        if !trie.starts_with(tmp.clone()) {
            return;
        } else if trie.search(tmp.clone()) {
            exists.insert(tmp.clone());
        }
        if i > 0 {
            Self::search(
                &board,
                i - 1,
                j,
                tmp.clone(),
                trie,
                &mut visited.clone(),
                exists,
            );
        }
        if i < board.len() - 1 {
            Self::search(
                &board,
                i + 1,
                j,
                tmp.clone(),
                trie,
                &mut visited.clone(),
                exists,
            );
        }
        if j > 0 {
            Self::search(
                &board,
                i,
                j - 1,
                tmp.clone(),
                trie,
                &mut visited.clone(),
                exists,
            );
        }
        if j < board[0].len() - 1 {
            Self::search(
                &board,
                i,
                j + 1,
                tmp.clone(),
                trie,
                &mut visited.clone(),
                exists,
            );
        }
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::find_words(vec![vec!['a', 'a']], vec!["a".to_string()]),
        vec!["a".to_string()]
    );
    assert_eq!(
        Solution::find_words(vec![vec!['a']], vec!["a".to_string()]),
        vec!["a".to_string()]
    );
    assert_eq!(
        Solution::find_words(
            vec![
                vec!['o', 'a', 'a', 'n'],
                vec!['e', 't', 'a', 'e'],
                vec!['i', 'h', 'k', 'r'],
                vec!['i', 'f', 'l', 'v']
            ],
            vec![
                "oath".to_string(),
                "pea".to_string(),
                "eat".to_string(),
                "rain".to_string()
            ]
        ),
        vec!["oath".to_string(), "eat".to_string()]
    );
}
