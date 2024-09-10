use std::collections::HashMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut hashes = HashMap::new();
        for row in grid.iter() {
            *hashes.entry(row).or_insert(0) += 1;
        }

        let mut counter = 0;
        for j in 0..grid.len() {
            let mut temp = vec![0; grid.len()];
            for i in 0..grid.len() {
                temp[i] = grid[i][j];
            }
            if let Some(&acc) = hashes.get(&temp) {
                counter += acc;
            }
        }

        counter
    }
}

// https://leetcode.com/problems/equal-row-and-column-pairs
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::equal_pairs(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]]),
            1
        );
    }

    #[test]
    fn example2() {
        let i = Solution::equal_pairs(vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 5],
            vec![2, 4, 2, 2],
            vec![2, 4, 2, 2],
        ]);
        assert_eq!(i, 3);
    }
}
