// https://leetcode.com/problems/majority-element/

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();

        for n in nums {
            map.entry(n)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        *map.iter().max_by_key(|entry| entry.1).unwrap().0
    }
}

// ################

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = Solution::majority_element(vec![3, 2, 3]);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let result = Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]);
        assert_eq!(result, 2);
    }
}
