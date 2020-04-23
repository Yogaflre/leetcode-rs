// <二分查找>
// 循环
fn loop_search(nums: Vec<i32>, target: i32) -> bool {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if nums[mid] == target {
            return true;
        } else if nums[mid] > target {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    return false;
}
// 递归
fn recursive_search(nums: Vec<i32>, target: i32, left: usize, right: usize) -> bool {
    if left > right {
        return false;
    }
    let mid = (left + right) / 2;
    if nums[mid] == target {
        return true;
    } else if nums[mid] > target {
        return recursive_search(nums, target, left, mid - 1);
    } else {
        return recursive_search(nums, target, mid + 1, right);
    }
}

// 查找第一个值等于给定的元素的下标
fn find_first(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let mut mid = (left + right) / 2;
        if nums[mid] == target {
            // 循环获取第一个元素
            while mid > left && nums[mid - 1] == target {
                mid = mid - 1;
            }
            return mid as i32;
        } else if nums[mid] > target {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    return -1;
}

#[test]
fn run() {
    let nums = vec![1, 3, 4, 5, 6, 8, 8, 8, 11, 18];

    // assert!(loop_search(nums, 61));

    // let len = nums.len();
    // assert!(recursive_search(nums, 8, 0, len - 1));

    assert_eq!(find_first(nums, 8), 5);
}
