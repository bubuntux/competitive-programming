#[derive(Debug)]
struct Line {
    result: usize,
    elements: Vec<usize>,
}

fn get_lines(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.split(':').collect::<Vec<&str>>())
        .filter(|split| !split.is_empty())
        .map(|split| Line {
            result: split[0].parse::<usize>().expect("parse result"),
            elements: split[1]
                .split_whitespace()
                .flat_map(|x| x.parse::<usize>())
                .collect::<Vec<usize>>(),
        })
        .collect::<Vec<Line>>()
}

#[allow(dead_code)]
fn part1(input: &str) -> usize {
    get_lines(input)
        .iter()
        .filter(|line| is_valid(line))
        .map(|line| line.result)
        .sum()
}

fn is_valid(line: &Line) -> bool {
    validate(line, 1, line.elements[0])
}

fn validate(line: &Line, i: usize, acc: usize) -> bool {
    if i >= line.elements.len() {
        return acc == line.result;
    }
    let sum = acc + line.elements[i];
    let mul = acc * line.elements[i];
    (sum <= line.result && validate(line, i + 1, sum))
        || (mul <= line.result && validate(line, i + 1, mul))
}

#[allow(dead_code)]
fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    const EXAMPLE: &str = "
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
            ";

    #[test]
    fn example1() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 3749);
    }

    #[test]
    fn answer1() {
        let input = fs::read_to_string("./src/day7/input").expect("read input");
        let result = part1(&input);
        print!("answer1 {}", result);
        assert_eq!(result, 5837374519342);
    }

    #[test]
    fn example2() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 0);
    }

    #[test]
    fn answer2() {
        let input = fs::read_to_string("./src/day7/input").expect("read input");
        let result = part2(&input);
        print!("answer2 {}", result);
    }
}
