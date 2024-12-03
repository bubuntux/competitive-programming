use regex::Regex;

#[allow(dead_code)]
fn part1(input: &str) -> isize {
    let re = Regex::new(r"mul\((-?\d{1,3}),(-?\d{1,3})\)").unwrap();
    re.captures_iter(input).fold(0, |mut sum, capture| {
        let (_, [x, y]) = capture.extract();
        let x = x.parse::<isize>().unwrap();
        let y = y.parse::<isize>().unwrap();
        sum += x * y;
        sum
    })
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn example1() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 161);
    }

    #[test]
    fn answer1() {
        let input = fs::read_to_string("./src/day3/input").expect("read input");
        let result = part1(&input);
        print!("answer1 {}", result);
        assert_eq!(result, 166357705);
    }
}
