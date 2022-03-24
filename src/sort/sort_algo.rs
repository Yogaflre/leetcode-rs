// <排序算法>
// https://mp.weixin.qq.com/s/jg9hLWhLnRhxHJ49H9E9ng

// 冒泡排序(每次调换左右位置，把大的放在后面)
fn bubble(nums: &mut Vec<i32>) {
    let mut flag = true;
    while flag {
        flag = false;
        for i in 1..nums.len() {
            if nums[i - 1] > nums[i] {
                nums.swap(i - 1, i);
                flag = true;
            }
        }
    }
}

// 插入排序(新元素->遍历已排序元素->插入)
fn insert(nums: &mut Vec<i32>) {
    for i in 1..nums.len() {
        for j in 0..i {
            if nums[i] < nums[j] {
                nums.swap(i, j);
            }
        }
    }
}

// 选择排序(新元素->在未排序序列里查找最小值->和新元素比较并替换)
fn select(nums: &mut Vec<i32>) {
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if nums[i] > nums[j] {
                nums.swap(i, j);
            }
        }
    }
}

// 快棑(找标志位->小的放左边；大的放右边—>递归)
fn fast(nums: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right {
        //递归终止条件：如果左右索引位置相同，则不需要排序
        return;
    }
    let mut l = left;
    let mut r = right;
    let piovt = nums[left]; //选择一个节点作为分割节点
    while l < r {
        //将小于piovt的节点都放在piovt左边，大于piovt的值都放在右边
        while nums[r] >= piovt && l < r {
            //NOTE 如果选取left为piovt，这里必须优先从右边开始遍历
            r -= 1;
        }
        while nums[l] <= piovt && l < r {
            l += 1;
        }
        if l < r {
            nums.swap(l, r);
        }
    }
    nums.swap(left, l); //将piovt替换到中点
    fast(nums, left, l - 1);
    fast(nums, l + 1, right);
}

// 归并排序(对半分割 -> 合并两个有序集合)
fn merge_split(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() <= 1 {
        return nums.clone();
    }
    let mid_index = nums.len() / 2;
    let left = merge_split(nums[..mid_index].to_vec());
    let right = merge_split(nums[mid_index..].to_vec());
    return do_merge(left, right);
}
fn do_merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    let mut l = 0;
    let mut r = 0;
    while l < left.len() && r < right.len() {
        if left[l] < right[r] {
            res.push(left[l]);
            l += 1;
        } else {
            res.push(right[r]);
            r += 1;
        }
    }
    if l < left.len() {
        res.extend_from_slice(&left[l..]);
    } else {
        res.extend_from_slice(&right[r..]);
    }
    return res;
}

#[test]
fn run() {
    let mut nums = vec![2, 9, 3, 4, 8, 3, 1];
    let mut sorted_nums = vec![1, 2, 3, 3, 4, 8, 9];
    bubble(&mut nums);
    // insert(&mut nums);
    // select(&mut nums);
    // let len = nums.len();
    // fast(&mut nums, 0, len - 1);
    // sorted_nums = merge_split(nums.clone());
    // assert_eq!(nums, sorted_nums);

    assert_eq!(nums, sorted_nums);
}
