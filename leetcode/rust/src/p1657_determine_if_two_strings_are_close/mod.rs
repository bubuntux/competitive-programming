use std::{
    char,
    collections::{HashMap, HashSet},
};

// https://leetcode.com/problems/determine-if-two-strings-are-close
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let char_count_1 = &word1.chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });
        let char_count_2 = &word2.chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });

        let unique_chars_1: HashSet<char> = char_count_1.keys().cloned().collect();
        let unique_chars_2: HashSet<char> = char_count_2.keys().cloned().collect();
        if !unique_chars_1.eq(&unique_chars_2) {
            return false;
        }

        char_count_1
            .values()
            .fold(HashMap::new(), |mut map, v| {
                *map.entry(*v).or_insert(0) += 1;
                map
            })
            .eq(&char_count_2.values().fold(HashMap::new(), |mut map, v| {
                *map.entry(*v).or_insert(0) += 1;
                map
            }))
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(Solution::close_strings(
            "abc".to_string(),
            "bca".to_string()
        ));
    }

    #[test]
    fn example_2() {
        assert!(!Solution::close_strings("a".to_string(), "aa".to_string()));
    }

    #[test]
    fn example_3() {
        assert!(Solution::close_strings(
            "cabbba".to_string(),
            "abbccc".to_string()
        ));
    }

    #[test]
    fn example_147() {
        assert!(!Solution::close_strings(
            "uau".to_string(),
            "ssx".to_string()
        ));
    }
}
