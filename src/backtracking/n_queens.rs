// <N皇后>

// The n-queens puzzle is the problem of placing n queens on an n×n chessboard such that no two queens attack each other.
// Given an integer n, return all distinct solutions to the n-queens puzzle.
// Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space respectively.

// Example:
// Input: 4
// Output: [
//  [".Q..",  // Solution 1
//   "...Q",
//   "Q...",
//   "..Q."],

//  ["..Q.",  // Solution 2
//   "Q...",
//   "...Q",
//   ".Q.."]
// ]
// Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above.

struct Solution;
impl Solution {
    /**
     * 回溯法(递归)
     * 使用递归，按行缩小问题的规模，并记录之前所有行的皇后位置
     * 递归入参：当前行、items[每行皇后的索引位置]、最终结果集
     * 递归终止条件：当前行==n时，将当前items写入结果集
     */
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = vec![];
        let items: Vec<usize> = vec![0; n as usize];
        Self::put(0, items, &mut result);
        return result;
    }

    fn put(row: usize, mut items: Vec<usize>, result: &mut Vec<Vec<String>>) {
        if row == items.len() {
            let mut tmp: Vec<String> = vec![];
            for i in 0..items.len() {
                let mut s = String::new();
                for j in 0..items.len() {
                    if j == items[i] {
                        s.push('Q');
                    } else {
                        s.push('.')
                    }
                }
                tmp.push(s);
            }
            result.push(tmp);
        } else if row < items.len() {
            for i in 0..items.len() {
                if Self::is_ok(row, i, &items) {
                    items[row] = i;
                    Self::put(row + 1, items.clone(), result);
                }
            }
        }
    }

    fn is_ok(row: usize, column: usize, items: &Vec<usize>) -> bool {
        for i in 0..row {
            if items[i] == column {
                return false;
            }
            if items[i] + (row - i) == column {
                return false;
            }
            if items[i] == column + (row - i) {
                return false;
            }
        }
        return true;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::solve_n_queens(4),
        vec![
            vec![
                ".Q..".to_string(),
                "...Q".to_string(),
                "Q...".to_string(),
                "..Q.".to_string()
            ],
            vec![
                "..Q.".to_string(),
                "Q...".to_string(),
                "...Q".to_string(),
                ".Q..".to_string()
            ]
        ]
    );
}
