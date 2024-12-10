use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    ops::Div,
};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_OREDRING: Regex = Regex::new(r"^(-?\d+)\|(-?\d+)").unwrap();
}
#[allow(dead_code)]
fn part1(input: &str) -> isize {
    let mut ordering: HashMap<String, HashSet<String>> = HashMap::new();

    let mut sum = 0;
    for line in input.trim().lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match RE_OREDRING.captures(line) {
            None => {
                let line: Vec<&str> = line.split(',').collect();
                sum += if is_valid(&ordering, &line) {
                    get_middle_value(&line)
                } else {
                    0
                }
            }
            Some(cap) => add_ordering(&mut ordering, cap),
        }
    }

    sum
}

fn is_valid(ordering: &HashMap<String, HashSet<String>>, line: &[&str]) -> bool {
    let processed: HashSet<String> = line.iter().map(|v| String::from(*v)).collect();

    line.is_sorted_by(|a, b| {
        !processed.contains(*a)
            || ordering
                .get(*b)
                .unwrap_or(&HashSet::new())
                .contains(&String::from(*a))
    })
}

fn fix(ordering: &HashMap<String, HashSet<String>>, line: &mut [&str]) {
    line.sort_by(|a, b| {
        if ordering
            .get(*b)
            .unwrap_or(&HashSet::new())
            .contains(&String::from(*a))
        {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
}

fn get_middle_value(line: &[&str]) -> isize {
    let middle_index = line.len().div(2);
    line[middle_index].parse::<isize>().unwrap()
}

fn add_ordering(ordering: &mut HashMap<String, HashSet<String>>, cap: regex::Captures<'_>) {
    ordering
        .entry(String::from(&cap[2]))
        .and_modify(|set| {
            set.insert(String::from(&cap[1]));
        })
        .or_insert(HashSet::from([String::from(&cap[1])]));
}

#[allow(dead_code)]
fn part2(input: &str) -> isize {
    let mut ordering: HashMap<String, HashSet<String>> = HashMap::new();

    let mut sum = 0;
    for line in input.trim().lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match RE_OREDRING.captures(line) {
            None => {
                let mut line: Vec<&str> = line.split(',').collect();
                if !is_valid(&ordering, &line) {
                    fix(&ordering, &mut line);
                    sum += get_middle_value(&line);
                }
            }
            Some(cap) => add_ordering(&mut ordering, cap),
        }
    }

    sum
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    const EXAMPLE: &str = "
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
            ";

    #[test]
    fn example1() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 143);
    }

    #[test]
    fn answer1() {
        let input = fs::read_to_string("./src/day5/input").expect("read input");
        let result = part1(&input);
        print!("answer1 {}", result);
        assert_eq!(result, 5955);
    }

    #[test]
    fn example2() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 123);
    }

    #[test]
    fn answer2() {
        let input = fs::read_to_string("./src/day5/input").expect("read input");
        let result = part2(&input);
        print!("answer2 {}", result);
        assert_eq!(result, 4030);
    }
}
