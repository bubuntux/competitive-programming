use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    pub fn find_single(nums: Vec<i32>) -> i32 {
        let mut elements: HashSet<i32> = HashSet::new();

        for n in nums {
            if !elements.remove(&n) {
                elements.insert(n);
            }
        }

        *elements.iter().last().unwrap()
    }
}

//

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::find_single(vec![2, 2, 1]), 1)
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::find_single(vec![4, 1, 2, 1, 2]), 4)
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::find_single(vec![1]), 1)
    }
}
