// <动态规划>

// 0-1背包问题(动态规划)：每次遍历一个层级的问题，并去重。第一层去重结果集可以推导出第二层结果集。直至问题规模缩小至完成
fn zero_one_package(items: &[i32], capacity: usize) -> usize {
    if items.is_empty() || capacity == 0 {
        return 0;
    }
    let mut bak: Vec<Vec<bool>> = vec![vec![false; capacity + 1]; items.len()];
    for i in 0..items.len() {
        if i == 0 {
            bak[i][0] = true;
            if items[0] as usize <= capacity {
                bak[i][items[0] as usize] = true;
            }
        } else {
            for j in 0..=capacity {
                if bak[i - 1][j] == true {
                    bak[i][j] = true;
                    if items[i] as usize <= capacity - j && bak[i][j + items[i] as usize] == false {
                        bak[i][j + items[i] as usize] = true;
                    }
                }
            }
        }
    }
    for j in (0..=capacity).rev() {
        if bak[items.len() - 1][j] == true {
            return j;
        }
    }
    return 0;
}
#[test]
fn zero_one_package_test() {
    assert_eq!(zero_one_package(&vec![2, 2, 4, 6, 3], 9), 9);
}

// 求解最短路径(medium::minimum_path_sum)

// 零钱兑换(medium::coin_change)
