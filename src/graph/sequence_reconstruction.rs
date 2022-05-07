// <重建序列>

use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn sequence_reconstruction(org: Vec<i32>, seqs: Vec<Vec<i32>>) -> bool {
        let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
        for seq in seqs {
            for i in 1..seq.len() {
                graph
                    .entry(seq[i - 1])
                    .or_insert(HashSet::new())
                    .insert(seq[i]);
            }
        }
        let mut flags: HashMap<i32, i32> = HashMap::new();
        for key in graph.keys() {
            flags.insert(*key, 0);
        }
        return Self::check(&graph, &org, 0, &mut flags);
    }

    // TODO
    fn check(
        graph: &HashMap<i32, HashSet<i32>>,
        org: &Vec<i32>,
        o_index: usize,
        flags: &mut HashMap<i32, i32>,
    ) -> bool {
        let target = org[o_index];
        false
    }
}

#[test]
fn test_sr() {
    let org = vec![1, 2, 3, 4];
    let seqs = vec![vec![1, 2, 3], vec![1, 2, 4], vec![3, 4]];
    assert_eq!(Solution::sequence_reconstruction(org, seqs), true);
}
