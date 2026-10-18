// https://leetcode.com/problems/majority-element/

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut count = 1;

        for i in 1..nums.len() {
            if count == 0 {
                res = nums[i];
                count = 1;
            } else if res == nums[i] {
                count += 1;
            } else {
                count -= 1;
            }
        }

        res
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
