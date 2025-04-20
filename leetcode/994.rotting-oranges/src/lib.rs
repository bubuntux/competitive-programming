#[allow(dead_code)]
impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut rottens: Vec<(usize, usize)> = vec![];
        let mut target = 0;
        for (y, x, v) in grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, v)| (y, x, *v)))
        {
            match v {
                2 => rottens.push((y, x)),
                1 => target += 1,
                _ => continue,
            }
        }
        let fresh_initial = target;
        let mut fresh_rotten = 0;
        let mut minutes = -1;
        let mut grid = grid;

        while !rottens.is_empty() {
            minutes += 1;
            let mut new_rottens = vec![];
            for (y, x) in rottens {
                // Up
                if y > 0 && grid[y - 1][x] == 1 {
                    grid[y - 1][x] = 2;
                    fresh_rotten += 1;
                    new_rottens.push((y - 1, x));
                }
                // Down
                if y < grid.len() - 1 && grid[y + 1][x] == 1 {
                    grid[y + 1][x] = 2;
                    fresh_rotten += 1;
                    new_rottens.push((y + 1, x));
                }
                // Left
                if x > 0 && grid[y][x - 1] == 1 {
                    grid[y][x - 1] = 2;
                    fresh_rotten += 1;
                    new_rottens.push((y, x - 1));
                }
                // Right
                if x < grid[y].len() - 1 && grid[y][x + 1] == 1 {
                    grid[y][x + 1] = 2;
                    fresh_rotten += 1;
                    new_rottens.push((y, x + 1));
                }
            }
            rottens = new_rottens;
        }

        if fresh_initial == fresh_rotten {
            minutes.max(0)
        } else {
            -1
        }
    }
}

// https://leetcode.com/problems/rotting-oranges

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]),
            4
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]),
            -1
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::oranges_rotting(vec![vec![0, 2]]), 0)
    }

    #[test]
    fn example_4() {
        assert_eq!(Solution::oranges_rotting(vec![vec![0]]), 0)
    }
}
