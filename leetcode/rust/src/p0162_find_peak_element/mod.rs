#[allow(dead_code)]
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            if (left == 0 || nums[left] > nums[left - 1]) && nums[left] > nums[left + 1] {
                return left as i32;
            }
            if nums[right] > nums[right - 1]
                && (right == nums.len() - 1 || nums[right] > nums[right + 1])
            {
                return right as i32;
            }
            left += 1;
            right -= 1;
        }

        0
    }
}

//
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exmaple_1() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
    }

    #[test]
    fn exmaple_2() {
        //TODO conditional assert? 2, or 6?
        assert_eq!(Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 1);
    }

    #[test]
    fn exmaple_3() {
        assert_eq!(Solution::find_peak_element(vec![1, 0, 1, 3, 5, 6, 4]), 0);
    }

    #[test]
    fn expample_4() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 1]), 1);
    }
}
