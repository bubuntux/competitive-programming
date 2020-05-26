use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (i, n) in nums.iter().enumerate() {
            match map.get(&(target - n)) {
                Some(index) => return vec![*index, i as i32],
                None => map.insert(*n, i as i32),
            };
        }

        unreachable!();
    }
}

//
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
