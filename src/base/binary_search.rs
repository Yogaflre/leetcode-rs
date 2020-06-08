// <二分查找>

// 循环
fn loop_search(nums: &Vec<i32>, target: i32) -> bool {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let mid = left + ((right - left) >> 1);
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
fn recursive_search(nums: &Vec<i32>, target: i32, left: usize, right: usize) -> bool {
    if left > right {
        return false;
    }
    let mid = left + ((right - left) >> 1);
    if nums[mid] == target {
        return true;
    } else if nums[mid] > target {
        return recursive_search(nums, target, left, mid - 1);
    } else {
        return recursive_search(nums, target, mid + 1, right);
    }
}

// 查找第一个值等于给定的元素的下标
fn find_first(nums: &Vec<i32>, target: i32) -> i32 {
    if !nums.is_empty() {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let mid = left + ((right - left) >> 1);
            if nums[mid] == target {
                // NOTE 向前寻找相同元素
                if mid > 0 && nums[mid - 1] == target {
                    left = mid - 1;
                    right = left;
                } else {
                    return mid as i32;
                }
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }
    return -1;
}

// 查到最后一个值等于给定元素的下标
fn find_end(nums: &Vec<i32>, target: i32) -> i32 {
    if !nums.is_empty() {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let mid = left + ((right - left) >> 1);
            if nums[mid] == target {
                // NOTE 向后寻找相同元素
                if mid < nums.len() - 1 && nums[mid + 1] == target {
                    left = mid + 1;
                    right = left;
                } else {
                    return mid as i32;
                }
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }
    return -1;
}

// 查找第一个大于等于给定值的元素
fn find_first_bigger(nums: &Vec<i32>, target: i32) -> i32 {
    if !nums.is_empty() {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let mid = left + ((right - left) >> 1);
            if nums[mid] >= target {
                // NOTE 判断前一个元素是否大于等于目标值
                if mid == 0 || nums[mid - 1] < target {
                    return nums[mid];
                } else {
                    right = mid - 1;
                }
            } else {
                left = mid + 1;
            }
        }
    }
    return -1;
}

// 查找最后一个小于给定值的元素
fn find_last_smaller(nums: &Vec<i32>, target: i32) -> i32 {
    if !nums.is_empty() {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let mid = left + ((right - left) >> 1);
            if nums[mid] <= target {
                // NOTE 判断后一个元素是否小于等于目标值
                if mid == nums.len() - 1 || nums[mid + 1] > target {
                    return nums[mid];
                } else {
                    left = mid + 1;
                }
            } else {
                right = mid - 1;
            }
        }
    }
    return -1;
}

#[test]
fn run() {
    let nums = vec![1, 3, 4, 5, 6, 8, 8, 8, 11, 18];
    assert_eq!(loop_search(&nums, 61), false);
    assert_eq!(recursive_search(&nums, 8, 0, nums.len() - 1), true);
    assert_eq!(find_first(&nums, 8), 5);
    assert_eq!(find_end(&nums, 8), 7);
    assert_eq!(find_first_bigger(&nums, 7), 8);
    assert_eq!(find_last_smaller(&nums, 7), 6);
}
