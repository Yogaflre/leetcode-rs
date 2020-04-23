// <回溯>

// 八皇后问题
fn eight_queen() -> [usize; 8] {
    let mut board: [usize; 8] = [0; 8];
    put(0, &mut board);
    return board;
}

fn put(row: usize, board: &mut [usize; 8]) -> bool {
    if row < board.len() {
        for column in 0..board.len() {
            if is_ok(board, row, column) {
                board[row] = column;
                // FIXME Rust栈深度不够
                if put(row + 1, board) {
                    return true;
                }
            }
        }
        return false;
    } else {
        return true;
    }
}
fn is_ok(board: &[usize; 8], row: usize, column: usize) -> bool {
    for i in 0..row {
        if board[i] == column {
            return false;
        }
        if board[i] + (row - i) == column {
            return false;
        }
        if board[i] - (row - i) == column {
            return false;
        }
    }
    return true;
}
#[test]
fn eight_queen_test() {
    assert_eq!(eight_queen(), [0, 4, 7, 5, 2, 6, 1, 3]);
}

// 0-1背包问题(回溯法：遍历所有可能的结果，取最符合题意的一个)
fn zero_one_package(items: &[i32], capacity: i32) -> Vec<i32> {
    if capacity <= 0 || items.len() == 0 {
        return vec![];
    }
    let mut result: Vec<i32> = vec![];
    for i in 0..items.len() {
        let mut tmp: Vec<i32> = vec![];
        tmp.push(items[i]);
        tmp.extend(zero_one_package(&items[i + 1..], capacity - items[i]));
        let sum = tmp.iter().sum::<i32>();
        if sum > result.iter().sum() && sum <= capacity {
            result = tmp;
        }
    }
    return result;
}
#[test]
fn zero_one_package_test() {
    assert_eq!(zero_one_package(&vec![9, 2, 12, 6], 16), vec![9, 9]);
}

// 匹配(*匹配任意字符0~无穷/?匹配任意字符0-1个)
fn rmatch(a: String, b: String) -> bool {
    rmatch_recursive(
        &a.chars().collect::<Vec<char>>(),
        0,
        &b.chars().collect::<Vec<char>>(),
        0,
    )
}
fn rmatch_recursive(a: &[char], ai: usize, b: &[char], bi: usize) -> bool {
    if b.len() == bi {
        if a.len() == ai {
            return true;
        } else {
            return false;
        }
    }
    if b[bi] == '*' {
        for i in 0..=(a.len() - ai) {
            if rmatch_recursive(a, ai + i, b, bi + 1) {
                return true;
            }
        }
    } else if b[bi] == '?' {
        if rmatch_recursive(a, ai, b, bi + 1) {
            return true;
        } else {
            return rmatch_recursive(a, ai + 1, b, bi + 1);
        }
    } else if ai < a.len() && a[ai] == b[bi] {
        return rmatch_recursive(a, ai + 1, b, bi + 1);
    }
    return false;
}
#[test]
fn rmatch_test() {
    assert_eq!(rmatch("aaa".to_string(), "a*a".to_string()), true);
    assert_eq!(rmatch("aaa".to_string(), "c*a*b?".to_string()), false);
}
