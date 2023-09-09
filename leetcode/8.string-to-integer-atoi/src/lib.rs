//https://leetcode.com/problems/string-to-integer-atoi

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut chars = s.trim_start().chars();
        let mut result = 0;
        let mut sign = 1;

        match chars.next() {
            Some('+') => (),
            Some('-') => sign = -1,
            Some(c @ '0'..='9') => result = c.to_digit(10).unwrap() as i32,
            _ => return result,
        }

        for c in chars {
            match c {
                '0'..='9' => {
                    result = result
                        .saturating_mul(10)
                        .saturating_add(sign * c.to_digit(10).unwrap() as i32)
                }
                _ => break,
            }
            if result == i32::MIN || result == i32::MAX {
                break;
            }
        }

        return result;
    }
}

// #############

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = Solution::my_atoi(String::from("42"));
        assert_eq!(result, 42);
    }

    #[test]
    fn example_2() {
        let result = Solution::my_atoi(String::from("     -42"));
        assert_eq!(result, -42);
    }

    #[test]
    fn example_3() {
        let result = Solution::my_atoi(String::from("4193 with words"));
        assert_eq!(result, 4193);
    }

    #[test]
    fn example_4() {
        let result = Solution::my_atoi(String::from("-91283472332"));
        assert_eq!(result, -2147483648);
    }
}
