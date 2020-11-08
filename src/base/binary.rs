// <二进制处理>

/**
 * 数字转二进制串
 */
fn num_to_binary(mut num: i8) -> String {
    let mut tmp: u8 = num as u8;
    // NOTE 如果是负数，则先切换为无符号的补码(取反+1)进行计算
    if num < 0 {
        tmp = (((num * -1) as u8) ^ u8::max_value()) + 1;
    }
    let mut res: Vec<char> = vec![];
    while tmp != 0 {
        res.push(std::char::from_digit((tmp % 2) as u32, 2).unwrap());
        tmp = tmp >> 1;
    }
    // NOTE 前面是按位从右向左依次添加，这里需要反转
    res.reverse();
    return res.iter().collect::<String>();
}

/**
 * 二进制串转数字
 */
fn binary_to_num(binary: String) -> i8 {
    let mut res: u8 = 0;
    let mut tmp: u16 = 1;
    let binarys: Vec<char> = binary.chars().collect();
    for c in binarys.into_iter().rev() {
        res += tmp as u8 * c.to_digit(10).unwrap() as u8;
        tmp *= 2;
    }
    // NOTE 将无符号数转为有符号数
    return res as i8;
}

#[test]
fn test_binary() {
    assert_eq!(num_to_binary(13), String::from("1101"));
    assert_eq!(num_to_binary(-10), String::from("11110110"));
    assert_eq!(binary_to_num(String::from("00001101")), 13);
    assert_eq!(binary_to_num(String::from("11110110")), -10);
}
