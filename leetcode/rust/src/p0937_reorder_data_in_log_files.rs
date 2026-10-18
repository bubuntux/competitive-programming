use std::cmp::Ordering;
use std::cmp::Ordering::*;

#[derive(Eq, PartialEq)]
struct Log {
    id: String,
    value: String,
}

impl Log {
    fn new(log: &str) -> Log {
        let index = log.find(' ').unwrap() + 1;
        let id = log.get(0..index).unwrap();
        let value = log.get(index..).unwrap();
        Log { id: id.to_string(), value: value.to_string() }
    }

    fn is_digit(&self) -> bool {
        self.value.chars().next().unwrap().is_ascii_digit()
    }
}

impl Ord for Log {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.value.cmp(&other.value) {
            Equal => { self.id.cmp(&other.id) }
            ord => ord
        }
    }
}

impl PartialOrd for Log {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ToString for Log {
    fn to_string(&self) -> String {
        format!("{}{}", self.id, self.value)
    }
}

impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut letters: Vec<Log> = Vec::new();
        let mut digits: Vec<String> = Vec::new();

        for x in logs {
            let log = Log::new(&x);
            if log.is_digit() {
                digits.push(x);
            } else {
                letters.push(log);
            }
        }

        letters.sort();

        let mut result: Vec<String> = Vec::new();
        for letter in letters.iter().map(Log::to_string) {
            result.push(letter);
        }
        result.extend(digits);

        result
    }
}

// https://leetcode.com/problems/reorder-data-in-log-files/

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::reorder_log_files(vec!["dig1 8 1 5 1".to_string(), "let1 art can".to_string(), "dig2 3 6".to_string(), "let2 own kit dig".to_string(), "let3 art zero".to_string()]),
            vec!["let1 art can", "let3 art zero", "let2 own kit dig", "dig1 8 1 5 1", "dig2 3 6"]
        );
    }
}
