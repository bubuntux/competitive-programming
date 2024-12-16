use core::str;

type Block = Option<usize>;

#[allow(dead_code)]
fn part1(input: &str) -> usize {
    let mut blocks = get_blocks(input);
    shift_left(&mut blocks);
    checksum(&blocks)
}

fn get_blocks(input: &str) -> Vec<Block> {
    let mut expanded = Vec::new();
    let mut id = 0;
    for (i, v) in input.trim().chars().enumerate() {
        let v = v.to_digit(10);
        let v = v.unwrap() as usize;
        let block = if i % 2 == 0 { Some(id) } else { None };
        if block.is_some() {
            id += 1;
        }
        expanded.resize(expanded.len() + v, block);
    }
    expanded
}

fn shift_left(blocks: &mut [Block]) {
    let (mut left, _) = blocks
        .iter()
        .enumerate()
        .find(|(_, b)| b.is_none())
        .unwrap();
    let (mut right, _) = blocks
        .iter()
        .enumerate()
        .rev()
        .find(|(_, b)| b.is_some())
        .unwrap();

    while left < right {
        blocks[left] = blocks[right];
        blocks[right] = None;
        (left, _) = blocks
            .iter()
            .enumerate()
            .skip(left)
            .find(|(_, b)| b.is_none())
            .unwrap();
        (right, _) = blocks
            .iter()
            .enumerate()
            .rev()
            .skip(blocks.len() - right)
            .find(|(_, b)| b.is_some())
            .unwrap();
    }
}

fn checksum(blocks: &[Block]) -> usize {
    blocks
        .iter()
        .flatten()
        .enumerate()
        .fold(0, |acc, (counter, block)| acc + (counter * block))
}

#[allow(dead_code)]
fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn example1() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 1928);
    }

    #[test]
    fn answer1() {
        let input = fs::read_to_string("./src/day9/input").expect("read input");
        let result = part1(&input);
        print!("answer1 {}", result);
        assert!(result > 89914538143);
        assert_eq!(result, 6340197768906);
    }

    #[test]
    fn example2() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 0);
    }

    #[test]
    fn answer2() {
        let input = fs::read_to_string("./src/day9/input").expect("read input");
        let result = part2(&input);
        print!("answer2 {}", result);
    }
}
