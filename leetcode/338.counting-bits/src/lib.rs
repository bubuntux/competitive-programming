#[allow(dead_code)]
impl Solution {
    pub fn count_bits(n: i32) -> Vec<usize> {
        //TODO

        vec![]
    }
}

struct Solution;

//
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
    }
    #[test]
    fn example_2() {
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
