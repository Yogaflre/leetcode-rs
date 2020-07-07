// <根据身高重建队列>

// Suppose you have a random list of people standing in a queue.
// Each person is described by a pair of integers (h, k), where h is the height of the person and k is the number of people in front of this person who have a height greater than or equal to h.
// Write an algorithm to reconstruct the queue.

// Note:
// The number of people is less than 1,100.

// Example
// Input:
// [[7,0], [4,4], [7,1], [5,0], [6,1], [5,2]]
// Output:
// [[5,0], [7,0], [5,2], [6,1], [4,4], [7,1]]

use std::cmp::Reverse;
struct Solution;
impl Solution {
    /**
     * 贪心算法
     * 先根据身高排序，并且保证同一身高内以升序排序
     * 再以p[1]为index插入新的数组中
     */
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        people.sort_by_key(|v| (Reverse(v[0]), v[1]));
        let mut res: Vec<Vec<i32>> = vec![];
        for p in people {
            res.insert(p[1] as usize, p);
        }
        return res;
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::reconstruct_queue(vec![vec![1, 0], vec![2, 0]]),
        vec![vec![1, 0], vec![2, 0]]
    );
    assert_eq!(
        Solution::reconstruct_queue(vec![
            vec![7, 0],
            vec![4, 4],
            vec![7, 1],
            vec![5, 0],
            vec![6, 1],
            vec![5, 2],
        ]),
        vec![
            vec![5, 0],
            vec![7, 0],
            vec![5, 2],
            vec![6, 1],
            vec![4, 4],
            vec![7, 1]
        ]
    );
}
