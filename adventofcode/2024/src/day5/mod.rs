use std::{
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
            None => sum += get_valid_mid_value(&ordering, line),
            Some(cap) => add_ordering(&mut ordering, cap),
        }
    }

    sum
}

fn get_valid_mid_value(ordering: &HashMap<String, HashSet<String>>, line: &str) -> isize {
    let split: Vec<&str> = line.split(',').collect();
    let processed: HashMap<String, usize> = split
        .iter()
        .enumerate()
        .map(|(i, v)| (String::from(*v), i))
        .collect();
    //  println!("processed = {:?}", processed);
    let mut valid = true;
    for (i, x) in split.iter().enumerate() {
        let x = String::from(*x);
        match ordering.get(&x) {
            None => continue,
            Some(o) => {
                //   println!("i={:?}, x={:?}", i, x);
                //   println!("ordering = {:?}", o);
                if !o.iter().all(|oe| {
                    let get = *processed.get(oe).unwrap_or(&0);
                    //         println!("oe={:?} get={:?}", oe, get);
                    get <= i
                }) {
                    valid = false;
                    //           println!("invalid");
                    break;
                }
            }
        }
    }

    if valid {
        let div_ceil = split.len().div(2);
        let parse = split[div_ceil].parse::<isize>().unwrap();
        // println!("middle i={:?}, value={:?}", div_ceil, parse);
        parse
    } else {
        0
    }
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
fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    #[test]
    fn example1_2() {
        let result = part1(
            "
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
            ",
        );
        assert_eq!(result, 143);
    }

    #[test]
    fn answer1() {
        let input = fs::read_to_string("./src/day5/input").expect("read input");
        let result = part1(&input);
        print!("answer1 {}", result);
    }

    #[test]
    fn example2() {
        let result = part2(
            "
            ",
        );
        assert_eq!(result, 9);
    }

    #[test]
    fn answer2() {
        let input = fs::read_to_string("./src/day5/input").expect("read input");
        let result = part2(&input);
        print!("answer2 {}", result);
    }
}
