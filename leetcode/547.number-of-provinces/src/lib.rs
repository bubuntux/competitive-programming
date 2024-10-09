use std::collections::hash_map::Entry::Vacant;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut islands: HashMap<usize, i32> = HashMap::new();
        is_connected
            .iter()
            .enumerate()
            .for_each(|(i, _)| Self::dfs(&is_connected, i, i, &mut islands));
        islands.values().collect::<HashSet<_>>().len() as i32
    }

    fn dfs(
        is_connected: &Vec<Vec<i32>>,
        i: usize,
        start: usize,
        islands: &mut HashMap<usize, i32>,
    ) {
        if let Vacant(entry) = islands.entry(i) {
            entry.insert(start as i32);
            is_connected[i].iter().enumerate().for_each(|(x, v)| {
                if *v == 1 {
                    Self::dfs(is_connected, x, start, islands);
                }
            });
        }
    }
}

// https://leetcode.com/problems/number-of-provinces/

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
            2
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::find_circle_num(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
            3
        );
    }
}
