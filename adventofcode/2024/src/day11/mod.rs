fn get_stones(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn blink(stones: &mut Vec<usize>) {
    let mut i = 0;
    while i < stones.len() {
        match stones[i] {
            0 => stones[i] = 1,
            stone if stone.to_string().len() % 2 == 0 => {
                let stone = stone.to_string();
                let middle = stone.len() / 2;
                let left = &stone[..middle].parse().unwrap();
                let right = &stone[middle..].parse().unwrap();
                stones[i] = *right;
                stones.insert(i, *left);
                i += 1;
            }
            _ => stones[i] *= 2024,
        }
        i += 1;
    }
}

#[allow(dead_code)]
fn part1(input: &str) -> usize {
    let mut stones = get_stones(input);
    for _ in 0..25 {
        blink(&mut stones);
    }
    stones.len()
}

#[allow(dead_code)]
fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    const EXAMPLE: &str = "125 17";

    #[test]
    fn example1() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 55312);
    }

    #[test]
    fn answer1() {
        let result = part1("30 71441 3784 580926 2 8122942 0 291");
        print!("answer1 {}", result);
        assert_eq!(result, 191690);
    }

    #[test]
    fn example2() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 0);
    }

    #[test]
    fn answer2() {
        let input = fs::read_to_string("./src/day11/input").expect("read input");
        let result = part2(&input);
        print!("answer2 {}", result);
    }
}
