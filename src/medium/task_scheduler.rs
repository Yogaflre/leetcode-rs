// <任务调度器>

// You are given a char array representing tasks CPU need to do. It contains capital letters A to Z where each letter represents a different task. Tasks could be done without the original order of the array. Each task is done in one unit of time. For each unit of time, the CPU could complete either one task or just be idle.
// However, there is a non-negative integer n that represents the cooldown period between two same tasks (the same letter in the array), that is that there must be at least n units of time between any two same tasks.
// You need to return the least number of units of times that the CPU will take to finish all the given tasks.

// Example 1:
// Input: tasks = ["A","A","A","B","B","B"], n = 2
// Output: 8
// Explanation:
// A -> B -> idle -> A -> B -> idle -> A -> B
// There is at least 2 units of time between any two same tasks.

// Example 2:
// Input: tasks = ["A","A","A","B","B","B"], n = 0
// Output: 6
// Explanation: On this case any permutation of size 6 would work since n = 0.
// ["A","A","A","B","B","B"]
// ["A","B","A","B","A","B"]
// ["B","B","B","A","A","A"]
// ...
// And so on.

// Example 3:
// Input: tasks = ["A","A","A","A","A","A","B","C","D","E","F","G"], n = 2
// Output: 16
// Explanation:
// One possible solution is
// A -> B -> C -> A -> D -> E -> A -> F -> G -> A -> idle -> idle -> A -> idle -> idle -> A

// Constraints:
// The number of tasks is in the range [1, 10000].
// The integer n is in the range [0, 100].

use std::cmp::max;
struct Solution;
impl Solution {
    /**
     * WHY 都没搞清楚自己怎么写的 = =
     * 先计算最高元素的个数total_tasks，并配合n的个数求得最高元素需要多少个task任务
     * 如果存在多个最高个数元素，则依次增加total_tasks个数
     * 当total_tasks大于所有任务数时，返回total_tasks；否则返回tasks任务数
     */
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut counts: Vec<usize> = vec![0; 26];
        let mut max_count = 0;
        for c in &tasks {
            let index = *c as usize - 'A' as usize;
            counts[index] += 1;
            max_count = max(counts[index], max_count);
        }
        let mut total_tasks = max_count + ((max_count - 1) * n as usize) - 1;
        for n in counts {
            if n == max_count {
                total_tasks += 1;
            }
        }
        if total_tasks >= tasks.len() {
            return total_tasks as i32;
        } else {
            return tasks.len() as i32;
        }
    }
}

#[test]
fn run() {
    assert_eq!(
        Solution::least_interval(
            vec![
                'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D',
                'E', 'F'
            ],
            4
        ),
        19
    );
    assert_eq!(Solution::least_interval(vec!['A', 'B', 'A', 'B'], 2), 5);
    assert_eq!(
        Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0),
        6
    );
    assert_eq!(
        Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2),
        8
    );
}
