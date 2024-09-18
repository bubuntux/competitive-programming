impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut zero_queue = Vec::new();
        let mut max_distance = 0;
        let mut left = -1;

        for right in 0..nums.len() {
            if nums[right] == 0 {
                zero_queue.push(right as i32);
            }
            if zero_queue.len() > k as usize {
                left = zero_queue.remove(0);
            }
            max_distance = max_distance.max(right as i32 - left);
        }

        max_distance
    }
}

// https://leetcode.com/problems/max-consecutive-ones-ii

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2),
            6
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::longest_ones(
                vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                3
            ),
            10
        );
    }

    #[test]
    fn example_43() {
        assert_eq!(Solution::longest_ones(vec![0, 0, 0, 1], 4), 4);
    }
}
