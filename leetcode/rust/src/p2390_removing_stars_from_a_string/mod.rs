impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut stack = String::new();
        for c in s.chars() {
            if c == '*' {
                stack.pop();
            } else {
                stack.push(c);
            }
        }

        stack
    }
}

struct Solution;

// https://leetcode.com/problems/removing-stars-from-a-string

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::remove_stars(String::from("leet**cod*e")), "lecoe");
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::remove_stars(String::from("erase*****")), "");
    }
}
