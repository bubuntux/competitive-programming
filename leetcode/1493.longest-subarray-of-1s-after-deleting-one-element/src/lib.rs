impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut zero_queue = Vec::new();
        let mut max_distance = 0;
        let mut left = -1;

        for right in 0..nums.len() {
            if nums[right] == 0 {
                zero_queue.push(right as i32)
            }
            if zero_queue.len() > 1 {
                left = zero_queue.remove(0);
            }

            max_distance = max_distance.max(right as i32 - left - 1);
        }

        max_distance
    }
}

// https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::longest_subarray(vec![1, 1, 0, 1]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]),
            5
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::longest_subarray(vec![1, 1, 1]), 2);
    }
}
