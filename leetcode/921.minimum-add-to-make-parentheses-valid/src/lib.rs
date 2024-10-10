impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut open = 0;
        let mut close = 0;

        for paren in s.chars() {
            if paren == '(' {
                open += 1;
            } else if open > 0 {
                open -= 1;
            } else {
                close += 1;
            }
        }
        open + close
    }
}

// https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::min_add_to_make_valid("())".to_string()), 1);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::min_add_to_make_valid("(((".to_string()), 3);
    }
}
