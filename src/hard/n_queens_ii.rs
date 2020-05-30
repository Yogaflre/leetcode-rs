// <N皇后2>

// The n-queens puzzle is the problem of placing n queens on an n×n chessboard such that no two queens attack each other.
// Given an integer n, return the number of distinct solutions to the n-queens puzzle.

// Example:
// Input: 4
// Output: 2
// Explanation: There are two distinct solutions to the 4-queens puzzle as shown below.
// [
//  [".Q..",  // Solution 1
//   "...Q",
//   "Q...",
//   "..Q."],

//  ["..Q.",  // Solution 2
//   "Q...",
//   "...Q",
//   ".Q.."]
// ]

struct Solution;
impl Solution {
    /**
     * 回溯法(递归)
     * 同hard::n_queens
     */
    pub fn total_n_queens(n: i32) -> i32 {
        let items: Vec<usize> = vec![0; n as usize];
        return Self::put(0, items);
    }

    fn put(row: usize, mut items: Vec<usize>) -> i32 {
        let mut tmp = 0;
        if row == items.len() {
            tmp = 1;
        } else {
            for i in 0..items.len() {
                if Self::is_ok(row, i, &items) {
                    items[row] = i;
                    tmp += Self::put(row + 1, items.clone());
                }
            }
        }
        return tmp;
    }

    fn is_ok(row: usize, column: usize, items: &Vec<usize>) -> bool {
        for i in 0..row {
            if items[i] == column
                || items[i] + (row - i) == column
                || items[i] == column + (row - i)
            {
                return false;
            }
        }
        return true;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::total_n_queens(4), 2);
}
