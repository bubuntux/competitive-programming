use std::collections::VecDeque;

impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        let mut sorted_deck = deck.clone();
        sorted_deck.sort_by(|a, b| b.cmp(a));

        let mut result = VecDeque::new();
        for x in sorted_deck {
            let front = result.pop_back();
            if front.is_some() {
                result.push_front(front.unwrap());
            }
            result.push_front(x);
        }

        result.into()
    }
}

// https://leetcode.com/problems/reveal-cards-in-increasing-order/

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::deck_revealed_increasing(vec![17, 13, 11, 2, 3, 5, 7]),
            vec![2, 13, 3, 11, 5, 17, 7]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::deck_revealed_increasing(vec![1, 1000]),
            vec![1, 1000]
        );
    }
}
