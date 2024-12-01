use std::collections::HashMap;

fn get_lists(input: &str) -> (Vec<usize>, Vec<usize>) {
    let elements = input
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
        .collect::<Vec<(usize, usize)>>();
    let left: Vec<usize> = elements.iter().map(|(l, _)| *l).collect();
    let right: Vec<usize> = elements.iter().map(|(_, r)| *r).collect();
    (left, right)
}

#[allow(dead_code)]
fn part1(input: &str) -> usize {
    let (mut left, mut right) = get_lists(input);

    left.sort();
    right.sort();

    let mut result = 0;
    for i in 0..left.len() {
        result += left[i].abs_diff(right[i]);
    }

    result
}

#[allow(dead_code)]
fn part2(input: &str) -> usize {
    let (left, right) = get_lists(input);

    let counts: HashMap<&usize, usize> = right.iter().fold(HashMap::new(), |mut acc, k| {
        acc.entry(k).and_modify(|v| *v += 1).or_insert(1);
        acc
    });

    left.iter().fold(0, |mut acc, n| {
        let times = counts.get(n).unwrap_or(&0);
        acc += n * times;
        acc
    })
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    #[test]
    fn example1() {
        let result = part1(
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
    fn answer1() {
        let input = fs::read_to_string("./src/day1/input").expect("read input");
        let result = part1(&input);
        print!("answer1 {}", result);
    }

    #[test]
    fn example2() {
        let result = part2(
            "3   4
             4   3
             2   5
             1   3
             3   9
             3   3",
        );
        assert_eq!(result, 31);
    }

    #[test]
    fn answer2() {
        let input = fs::read_to_string("./src/day1/input").expect("read input");
        let result = part2(&input);
        print!("answer2 {}", result);
    }
}
