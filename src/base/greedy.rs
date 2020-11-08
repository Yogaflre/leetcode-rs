// <贪心算法>

// 分配糖果
fn distribute_candy(mut candies: i32, mut children: Vec<i32>) -> i32 {
    children.sort();
    let mut max = 0;
    for n in children {
        candies = candies - n;
        if candies >= 0 {
            max = max + 1;
        }
    }
    return max;
}
#[test]
fn test_distribute_candy() {
    // 糖果总数
    let candies = 20;
    // 代表每个孩子对糖果的需求个数
    let children = vec![4, 1, 2, 4, 7, 8, 10];
    assert_eq!(distribute_candy(candies, children), 5);
}

// 从n个区间中选出最多不相交的区间
fn find_disjoint_area(mut areas: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    areas.sort_by_key(|x| x.0);
    let mut result: Vec<(i32, i32)> = vec![];
    for i in 0..areas.len() {
        let mut tmp = areas[i];
        if result.is_empty() || tmp.0 >= result[result.len() - 1].1 {
            for j in i..areas.len() {
                if areas[j].1 < tmp.1 {
                    tmp = areas[j];
                }
            }
            result.push(tmp);
        }
    }
    return result;
}
#[test]
fn test_find_disjoint_area() {
    let areas = vec![(6, 8), (2, 4), (3, 5), (1, 5), (5, 9), (8, 10)];
    assert_eq!(find_disjoint_area(areas), vec![(2, 4), (6, 8), (8, 10)]);
}

fn remove_num(num: u32) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    let nums: Vec<char> = num.to_string().chars().collect();
    if nums.len() < 1 {
        return result;
    }
    let mut min: u32 = nums[0].to_digit(10).unwrap();
    for n in nums[1..].iter() {
        let n = n.to_digit(10).unwrap();
        if n > min {
            result.push(n);
        } else {
            result.push(min);
            min = n;
        }
    }
    result
}
#[test]
fn test_remove_num() {
    assert_eq!(remove_num(74615), vec![7, 6, 4, 5]);
}

// 跳跃游戏(medium::jump_game)
