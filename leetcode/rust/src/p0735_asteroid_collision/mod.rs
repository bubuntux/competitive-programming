use std::cmp::Ordering;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        for a in asteroids {
            result.push(a);
            while result.len() > 1 && result[result.len() - 2] > 0 && result[result.len() - 1] < 0 {
                let last = result.pop().unwrap();
                let prev_last = result.pop().unwrap();
                match prev_last.abs().cmp(&last.abs()) {
                    Ordering::Greater => result.push(prev_last),
                    Ordering::Less => result.push(last),
                    Ordering::Equal => {}
                }
            }
        }
        result
    }
}

// https://leetcode.com/problems/asteroid-collision/

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::asteroid_collision(vec![8, -8]), vec![]);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10]);
    }

    #[test]
    fn example_14() {
        assert_eq!(Solution::asteroid_collision(vec![-2, -1, 1, 2]), vec![-2, -1, 1, 2]);
    }

    #[test]
    fn example_80() {
        assert_eq!(Solution::asteroid_collision(vec![-2, -2, 1, -2]), vec![-2, -2, -2]);
    }
}
