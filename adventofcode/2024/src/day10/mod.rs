use std::{collections::HashMap, usize};

#[allow(dead_code)]
fn part1(input: &str) -> usize {
    let matrix = get_matrix(input);
    matrix
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, cell)| (y, x, cell)))
        .filter(|(_, _, cell)| *cell == &0)
        .fold(0, |sum, (y, x, _)| sum + score(&matrix, y, x))
}

fn score(matrix: &[Vec<usize>], y: usize, x: usize) -> usize {
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    iter(&mut visited, matrix, y, x, 1);
    visited.values().filter(|cell| *cell == &9).count()
}

fn iter(
    visited: &mut HashMap<(usize, usize), usize>,
    matrix: &[Vec<usize>],
    y: usize,
    x: usize,
    target: usize,
) {
    if visited.insert((y, x), matrix[y][x]).is_some() {
        return;
    }

    let new_y = y + 1;
    if new_y < matrix.len() && matrix[new_y][x] == target {
        iter(visited, matrix, new_y, x, target + 1);
    }
    let (new_y, overflow) = y.overflowing_sub(1);
    if !overflow && matrix[new_y][x] == target {
        iter(visited, matrix, new_y, x, target + 1);
    }
    let new_x = x + 1;
    if new_x < matrix[y].len() && matrix[y][new_x] == target {
        iter(visited, matrix, y, new_x, target + 1);
    }
    let (new_x, overflow) = x.overflowing_sub(1);
    if !overflow && matrix[y][new_x] == target {
        iter(visited, matrix, y, new_x, target + 1);
    }
}

fn get_matrix(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

#[allow(dead_code)]
fn part2(input: &str) -> usize {
    let matrix = get_matrix(input);
    matrix
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, cell)| (y, x, cell)))
        .filter(|(_, _, cell)| *cell == &0)
        .fold(0, |sum, (y, x, _)| sum + rank(&matrix, y, x))
}

fn rank(matrix: &[Vec<usize>], y: usize, x: usize) -> usize {
    iter2(matrix, y, x, 1)
}

fn iter2(matrix: &[Vec<usize>], y: usize, x: usize, target: usize) -> usize {
    if matrix[y][x] == 9 {
        return 1;
    }

    let mut paths = 0;
    let new_y = y + 1;
    if new_y < matrix.len() && matrix[new_y][x] == target {
        paths += iter2(matrix, new_y, x, target + 1);
    }
    let (new_y, overflow) = y.overflowing_sub(1);
    if !overflow && matrix[new_y][x] == target {
        paths += iter2(matrix, new_y, x, target + 1);
    }
    let new_x = x + 1;
    if new_x < matrix[y].len() && matrix[y][new_x] == target {
        paths += iter2(matrix, y, new_x, target + 1);
    }
    let (new_x, overflow) = x.overflowing_sub(1);
    if !overflow && matrix[y][new_x] == target {
        paths += iter2(matrix, y, new_x, target + 1);
    }
    paths
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    const EXAMPLE: &str = "
                    89010123
                    78121874
                    87430965
                    96549874
                    45678903
                    32019012
                    01329801
                    10456732
                    ";
    #[test]
    fn example1() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 36);
    }

    #[test]
    fn answer1() {
        let input = fs::read_to_string("./src/day10/input").expect("read input");
        let result = part1(&input);
        print!("answer1 {}", result);
        assert_eq!(result, 582);
    }

    #[test]
    fn example2() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 81);
    }

    #[test]
    fn answer2() {
        let input = fs::read_to_string("./src/day10/input").expect("read input");
        let result = part2(&input);
        print!("answer2 {}", result);
        assert_eq!(result, 1302);
    }
}
