// <匹配算法>

// 暴力搜索
fn bp(main: &str, aim: &str) -> bool {
    let mains: Vec<char> = main.chars().collect();
    let aims: Vec<char> = aim.chars().collect();
    for i in 0..=(mains.len() - aims.len()) {
        let mut tmp = true;
        for j in 0..aim.len() {
            if aims[j] != mains[i + j] {
                tmp = false;
                break;
            }
        }
        if tmp {
            return true;
        }
    }
    return false;
}
#[test]
fn bp_test() {
    assert_eq!(bp("agracghrewrqa", "ghr"), true);
    assert_eq!(bp("agracghrewrqa", "ghc"), false);
}

// rk(hash)
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
fn rk(main: &str, aim: &str) -> bool {
    // 构建字符字典表
    let dict: HashMap<char, i32> = HashMap::from_iter::<Vec<(char, i32)>>(vec![
        ('a', 0),
        ('b', 1),
        ('c', 2),
        ('d', 3),
        ('e', 4),
    ]);
    // 构建所有字符的hash码
    let mut hash_code: HashSet<i32> = HashSet::new();
    for i in 0..=main.len() - aim.len() {
        let tmp: &str = &main[i..i + aim.len()];
        hash_code.insert(hash(tmp, &dict));
    }
    // 匹配hash码
    return hash_code.contains(&hash(&aim, &dict));
}
// 求RK-HASH(使用进制的思想)
fn hash(text: &str, dict: &HashMap<char, i32>) -> i32 {
    let mut result: i32 = 0;
    let mut n = text.len() - 1;
    for t in text.chars() {
        let mut tmp: i32 = *dict.get(&t).unwrap();
        for _ in 0..n {
            tmp += tmp * 5;
            n -= 1;
        }
        result += tmp;
    }
    return result;
}
#[test]
fn rk_test() {
    assert_eq!(rk("abcde", "cd"), true);
    assert_eq!(rk("abcde", "dc"), false);
}

// Boyer-Moore(在bp的基础上找规律，尽量少的匹配校验)
fn bm(a: &str, b: &str) -> bool {
    let mains: Vec<char> = a.chars().collect();
    let aims: Vec<char> = b.chars().collect();
    let mut i = aims.len() - 1;
    let mut j = i;

    while i < mains.len() {
        if mains[i] == aims[j] {
            // 如果字符相同则同时移动
            if j == 0 {
                return true;
            }
            i -= 1;
            j -= 1;
        } else {
            i = bad_char(&mains, &aims, i, j);
            j = aims.len() - 1;
        }
    }
    return false;
}
// 坏字符规则
fn bad_char(mains: &Vec<char>, aims: &Vec<char>, i: usize, j: usize) -> usize {
    let mut tmp: usize = j;
    // TODO 寻找aims中是否存在i位置的字符位置(可以使用散列表优化)
    for k in (0..j).rev() {
        if aims[k] == mains[i] {
            tmp = k;
        }
    }
    if tmp == j {
        return i + aims.len();
    } else {
        return i + aims.len() - 1 - tmp;
    }
}
// TODO 好后缀规则
fn good_suffix(mains: &Vec<char>, aims: &Vec<char>, i: usize, j: usize) -> usize {
    0
}
#[test]
fn bm_test() {
    assert_eq!(bm("agracghrewrqa", "ghr"), true);
    assert_eq!(bm("agracghrewrqa", "ghc"), false);
    assert_eq!(bm("aaaaaaaaaaaaaa", "baaa"), false);
}

// TODO KMP算法
fn kmp(a: &str, b: &str) -> bool {
    false
}
#[test]
fn kmp_test() {
    assert_eq!(kmp("agracghrewrqa", "ghr"), true);
    assert_eq!(kmp("agracghrewrqa", "ghc"), false);
    assert_eq!(kmp("aaaaaaaaaaaaaa", "baaa"), false);
}
