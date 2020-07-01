use std::cmp::{max, min};

impl Solution {
    pub fn max_area(hs: Vec<i32>) -> i32 {
        let mut li: usize = 0;
        let mut ri: usize = hs.len() - 1;
        let mut res = min(hs[li], hs[ri]) * (ri - li) as i32;

        while ri - li > 1 {
            if hs[ri] > hs[li] {
                li += 1;
            } else {
                ri -= 1;
            }
            res = max(res, min(hs[li], hs[ri]) * (ri - li) as i32);
        }

        res
    }
}

// https://leetcode.com/problems/container-with-most-water/

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
