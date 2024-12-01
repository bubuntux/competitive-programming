#[allow(dead_code)]
fn process(input: &str) -> usize {
    let elements: Vec<(usize, usize)> = input
        .lines()
        .map(|line| -> (usize, usize) {
            let mut split = line.split_whitespace();
            let left = split
                .next()
                .expect("first element")
                .parse::<usize>()
                .unwrap();
            let right = split
                .next()
                .expect("second element")
                .parse::<usize>()
                .unwrap();
            (left, right)
        })
        .collect();

    let n = elements.len();
    let mut left: Vec<usize> = elements.iter().map(|(l, _)| *l).collect();
    let mut right: Vec<usize> = elements.iter().map(|(_, r)| *r).collect();

    left.sort();
    right.sort();

    let mut result = 0;
    for i in 0..n {
        result += left[i].abs_diff(right[i]);
    }

    result
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    #[test]
    fn example() {
        let result = process(
            "3   4
             4   3
             2   5
             1   3
             3   9
             3   3",
        );
        assert_eq!(result, 11);
    }

    #[test]
    fn answer() {
        let input = fs::read_to_string("./src/day1/input").expect("Unable to read input");
        let result = process(&input);
        print!("result {}", result);
    }
}
