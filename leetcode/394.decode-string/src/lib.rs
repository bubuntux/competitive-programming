impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack = Vec::new();
        let mut result = String::new();
        let mut k = 0;
        s.chars().for_each(|c| match c {
            '[' => {
                stack.push((k, result.clone()));
                k = 0;
                result.clear();
            }
            ']' => {
                let (last_k, last_result) = stack.pop().unwrap();
                result = result.repeat(last_k);
                result.insert_str(0, &last_result);
            }
            c => {
                if c.is_digit(10) {
                    k = 10 * k + c.to_digit(10).unwrap() as usize
                } else {
                    result.push(c)
                }
            }
        });

        result
    }
}

// https://leetcode.com/problems/decode-string/
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::decode_string(String::from("3[a]2[bc]")),
            "aaabcbc"
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::decode_string(String::from("3[a2[c]]")),
            "accaccacc"
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::decode_string(String::from("2[abc]3[cd]ef")),
            "abcabccdcdcdef"
        );
    }
}
