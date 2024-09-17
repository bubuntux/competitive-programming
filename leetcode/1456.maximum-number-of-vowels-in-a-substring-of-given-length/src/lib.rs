impl Solution {
    fn is_vowel(c: char) -> bool {
        match c {
            'a' => true,
            'e' => true,
            'i' => true,
            'o' => true,
            'u' => true,
            _ => false,
        }
    }

    pub fn max_vowels(s: String, k: i32) -> i32 {
        let count = s
            .chars()
            .take(k as usize)
            .filter(|x| Self::is_vowel(*x))
            .count();
        s.chars()
            .zip(s.chars().skip(k as usize))
            .fold((count, count), |(max, count), (left, right)| {
                match (Self::is_vowel(left), Self::is_vowel(right)) {
                    (false, true) => (max.max(count + 1), count + 1),
                    (true, false) => (max, count - 1),
                    _ => (max, count),
                }
            })
            .0 as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::max_vowels("abciiidef".to_string(), 3), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::max_vowels("aeiou".to_string(), 2), 2);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::max_vowels("leetcode".to_string(), 3), 2);
    }
}
