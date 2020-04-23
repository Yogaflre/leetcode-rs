// <排序算法>
// https://mp.weixin.qq.com/s/jg9hLWhLnRhxHJ49H9E9ng

// 冒泡排序(每次调换左右位置)
fn bubble(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 0..nums.len() {
        let mut flag = true;
        for j in 0..(nums.len() - i - 1) {
            if nums[j] > nums[j + 1] {
                flag = false;
                let tmp = nums[j];
                nums[j] = nums[j + 1];
                nums[j + 1] = tmp;
            }
        }
        println!("{:?}", nums);
        if flag {
            break;
        }
    }
    nums
}

// 插入排序(新元素->遍历已排序元素->插入)
fn insert(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in (0..=i).rev() {
            if nums[j] < nums[j - 1] {
                let tmp = nums[j];
                nums[j] = nums[j - 1];
                nums[j - 1] = tmp;
            }
        }
        println!("{:?}", nums);
    }
    nums
}

// 选择排序(新元素->在未排序序列里查找最小值->和新元素比较并替换)
fn select(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 0..nums.len() {
        let mut min_index = i;
        for j in i + 1..nums.len() {
            if nums[j] < nums[min_index] {
                min_index = j;
            }
        }
        if i != min_index {
            let tmp = nums[min_index];
            nums[min_index] = nums[i];
            nums[i] = tmp;
        }
        println!("{:?}", nums);
    }
    nums
}

// 快棑(找标志位->小的放左边；大的放右边—>递归)
fn fast(nums: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right {
        return;
    }
    // 随便选一个位置作为标志位
    let mid = nums[left];
    let mut l = left;
    let mut r = right;
    while l < r {
        // NOTE 很关键！如果选第一个元素为标志位，优先遍历r值
        while nums[r] >= mid && l < r {
            r -= 1;
        }
        while nums[l] <= mid && l < r {
            l += 1;
        }
        if l < r {
            let tmp = nums[l];
            nums[l] = nums[r];
            nums[r] = tmp;
        }
    }
    nums[left] = nums[l];
    nums[l] = mid;
    fast(nums, left, l - 1);
    fast(nums, l + 1, right);
}

// 归并排序(对半分割->)
fn merge(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.len() < 2 {
        return nums;
    }
    let mid_index = nums.len() / 2;
    let a = nums[..mid_index].to_vec();
    let b = nums[mid_index..].to_vec();
    let left = merge(a);
    let right = merge(b);
    return do_merge(left, right);
}
fn do_merge(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let mut a_index = 0;
    let mut b_index = 0;
    while a_index < a.len() && b_index < b.len() {
        if a[a_index] <= b[b_index] {
            result.push(a[a_index]);
            a_index += 1;
        } else {
            result.push(b[b_index]);
            b_index += 1;
        }
    }
    if a_index < a.len() {
        result.extend(a[a_index..].to_vec());
    } else {
        result.extend(b[b_index..].to_vec())
    }
    return result;
}

#[test]
fn run() {
    let original = vec![2, 9, 3, 4, 8, 3, 1];
    let sorted = vec![1, 2, 3, 3, 4, 8, 10];

    // assert_eq!(bubble(original.clone()), sorted);
    // assert_eq!(insert(original.clone()), sorted);
    // assert_eq!(select(original.clone()), sorted);

    // let mut fast_original = original.clone();
    // let right = fast_original.len() - 1;
    // fast(&mut fast_original, 0, right);
    // assert_eq!(fast_original, sorted);

    assert_eq!(merge(original.clone()), sorted);
}
