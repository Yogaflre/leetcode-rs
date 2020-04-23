// <霍夫曼编码>

use std::cmp::Reverse;
use std::collections::HashMap;
use std::iter::FromIterator;
fn huffman_coding(text: String) -> String {
    // 统计频率
    let mut count_table: HashMap<char, i32> = HashMap::default();
    let mut code_table: HashMap<char, String> = HashMap::default();
    for i in text.chars() {
        match count_table.get(&i) {
            Some(n) => count_table.insert(i, n + 1),
            None => count_table.insert(i, 1),
        };
    }
    // 排序
    let mut sort_count_table: Vec<(char, i32)> = Vec::from_iter(count_table);
    sort_count_table.sort_by_key(|x| Reverse(x.1));
    let sort_char: Vec<char> = sort_count_table.iter().map(|x| x.0).collect();

    //FIXME 构建“编码表”
    let mut tmp_code: String = String::from("1");
    for c in sort_char {
        if code_table.is_empty() {
            code_table.insert(c, tmp_code.clone());
        } else {
            tmp_code = "0".to_string() + &tmp_code;
            code_table.insert(c, tmp_code.clone());
        }
    }

    //压缩
    let mut result = String::from("");
    for c in text.chars() {
        result.push_str(code_table.get(&c).unwrap());
    }
    return result;
}

#[test]
fn test_huffman_coding() {
    assert_eq!(huffman_coding("ABBCADB".to_string()), "01110010100011");
}
