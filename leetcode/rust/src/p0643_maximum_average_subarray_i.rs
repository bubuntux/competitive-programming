impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum: f64 = nums.iter().map(|x| *x as f64).take(k as usize).sum();
        let mut max_sum = sum;

        for i in k as usize..nums.len() {
            sum = sum - nums[i - k as usize] as f64 + nums[i] as f64;
            max_sum = max_sum.max(sum);
        }

        max_sum / k as f64
    }
}

// https://leetcode.com/problems/maximum-average-subarray-i

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
            12.75000
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::find_max_average(vec![5], 1), 5.00000);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::find_max_average(vec![0, 1, 1, 3, 3], 4), 2.00000);
    }
}
