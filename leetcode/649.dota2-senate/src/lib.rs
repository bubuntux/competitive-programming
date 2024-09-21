use std::collections::VecDeque;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut r = VecDeque::new();
        let mut d = VecDeque::new();

        for (i, c) in senate.char_indices() {
            if c == 'R' {
                r.push_back(i);
            } else {
                d.push_back(i);
            }
        }

        while !r.is_empty() && !d.is_empty() {
            let ri = r.pop_front().unwrap();
            let di = d.pop_front().unwrap();
            if ri < di {
                r.push_back(ri + senate.len())
            } else {
                d.push_back(di + senate.len())
            }
        }

        if r.is_empty() {
            "Dire".to_string()
        } else {
            "Radiant".to_string()
        }
    }
}

// https://leetcode.com/problems/dota2-senate/

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::predict_party_victory("RD".to_string()), "Radiant");
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::predict_party_victory("RDD".to_string()), "Dire");
    }

    #[test]
    fn example_74() {
        assert_eq!(Solution::predict_party_victory("DDRRR".to_string()), "Dire");
    }
}
