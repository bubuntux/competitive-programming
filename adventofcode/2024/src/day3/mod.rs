use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MUL_RE: Regex = Regex::new(r"mul\((-?\d{1,3}),(-?\d{1,3})\)").unwrap();
    static ref DO_RE: Regex =
        Regex::new(r"(?:mul\((-?\d{1,3}),(-?\d{1,3})\))|(?:do\(\)()())|(?:don't\(\)()())").unwrap();
}

#[allow(dead_code)]
fn part1(input: &str) -> isize {
    MUL_RE.captures_iter(input).fold(0, |mut sum, capture| {
        let (_, [x, y]) = capture.extract();
        let x = x.parse::<isize>().unwrap();
        let y = y.parse::<isize>().unwrap();
        sum += x * y;
        sum
    })
}

#[allow(dead_code)]
fn part2(input: &str) -> isize {
    let mut process = true;
    DO_RE.captures_iter(input).fold(0, |mut sum, capture| {
        let (full, [x, y]) = capture.extract();
        match full {
            "do()" => process = true,
            "don't()" => process = false,
            _ => {
                if process {
                    let x = x.parse::<isize>().unwrap();
                    let y = y.parse::<isize>().unwrap();
                    sum += x * y;
                }
            }
        }
        sum
    })
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    #[test]
    fn example1() {
        let result =
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, 161);
    }

    #[test]
    fn answer1() {
        let input = fs::read_to_string("./src/day3/input").expect("read input");
        let result = part1(&input);
        print!("answer1 {}", result);
        assert_eq!(result, 166357705);
    }

    #[test]
    fn example2() {
        let result =
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, 48);
    }

    #[test]
    fn answer2() {
        let input = fs::read_to_string("./src/day3/input").expect("read input");
        let result = part2(&input);
        assert!(result > 59275940);
        print!("answer2 {}", result);
        assert_eq!(result, 88811886);
    }
}
