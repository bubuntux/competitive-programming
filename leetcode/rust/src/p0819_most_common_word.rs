use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut freq_map: HashMap<String, i32> = HashMap::new();
        let ban_set: HashSet<String> = HashSet::from_iter(banned);
        for word in paragraph.split(|c: char| c.is_ascii_punctuation() || c.is_ascii_whitespace()) {
            let word = word.to_ascii_lowercase();
            if word == "" || ban_set.contains(&word) {
                continue;
            }
            *freq_map.entry(word).or_insert(0) += 1;
        }
        freq_map
            .iter()
            .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
            .map(|(k, _)| k)
            .unwrap()
            .to_string()
    }
}

// https://leetcode.com/problems/most-common-word/
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::most_common_word(
                "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
                vec!["hit".to_string()]
            ),
            "ball"
        );
    }
}
