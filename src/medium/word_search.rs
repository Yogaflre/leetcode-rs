// <单词搜索>
// Given a 2D board and a word, find if the word exists in the grid.
// The word can be constructed from letters of sequentially adjacent cell, where "adjacent" cells are those horizontally or vertically neighboring. The same letter cell may not be used more than once.

// Example:
// board =
// [
//   ['A','B','C','E'],
//   ['S','F','C','S'],
//   ['A','D','E','E']
// ]
// Given word = "ABCCED", return true.
// Given word = "SEE", return true.
// Given word = "ABCB", return false.

// 解题思路
// 方法一：近似动态规划
//  依次遍历每个节点，并且每次创建新的visted二维数组(用来记录该节点是否被访问过)
//  每次递归先将visited[i][j]设置为true，如果递归结果为false则修改visited为false(这一点很重要了，为了可以重复遍历该字符)
struct Solution;
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let words: Vec<char> = word.chars().collect();
        let mut visted: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Self::exist_recursive(&board, &words, i, j, 0, &mut visted) {
                    return true;
                }
            }
        }
        return false;
    }

    fn exist_recursive(
        board: &Vec<Vec<char>>,
        words: &Vec<char>,
        i: usize,
        j: usize,
        index: usize,
        visted: &mut Vec<Vec<bool>>,
    ) -> bool {
        if board[i][j] != words[index] || visted[i][j] {
            return false;
        } else if index + 1 == words.len() {
            return true;
        }

        // NOTE 当所有条件不符合时，将该节点访问顺序置为false
        if i > 0 && index + 1 < words.len() {
            visted[i][j] = true;
            if Self::exist_recursive(board, words, i - 1, j, index + 1, visted) {
                return true;
            } else {
                visted[i][j] = false;
            }
        }

        if i + 1 < board.len() && index + 1 < words.len() {
            visted[i][j] = true;
            if Self::exist_recursive(board, words, i + 1, j, index + 1, visted) {
                return true;
            } else {
                visted[i][j] = false;
            }
        }

        if j > 0 && index + 1 < words.len() {
            visted[i][j] = true;
            if Self::exist_recursive(board, words, i, j - 1, index + 1, visted) {
                return true;
            } else {
                visted[i][j] = false;
            }
        }

        if j + 1 < board[0].len() && index + 1 < words.len() {
            visted[i][j] = true;
            if Self::exist_recursive(board, words, i, j + 1, index + 1, visted) {
                return true;
            } else {
                visted[i][j] = false;
            }
        }
        return false;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCCED".to_string()
        ),
        true
    );
    assert_eq!(
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "SEE".to_string()
        ),
        true
    );
    assert_eq!(
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCB".to_string()
        ),
        false
    );
    assert_eq!(
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'E', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCESEEEFS".to_string()
        ),
        true
    );
    assert_eq!(Solution::exist(vec![vec!['A']], "A".to_string()), true);
}
