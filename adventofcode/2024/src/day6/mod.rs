#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone)]
enum Cell {
    Empty,
    Obstruction,
    Guard(Direction),
    Visited,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[allow(dead_code)]
fn part1(input: &str) -> usize {
    let mut matrix = get_matrix(input);

    let gp = guard_point(&matrix);
    if gp.is_none() {
        return 0;
    }

    let mut gp = gp.unwrap();
    let mut np = next_point(&matrix, gp);

    while let Some(npv) = np {
        match matrix[npv.y][npv.x] {
            Cell::Obstruction => {
                if let Cell::Guard(dir) = matrix[gp.y][gp.x] {
                    matrix[gp.y][gp.x] = match dir {
                        Direction::Up => Cell::Guard(Direction::Right),
                        Direction::Right => Cell::Guard(Direction::Down),
                        Direction::Down => Cell::Guard(Direction::Left),
                        Direction::Left => Cell::Guard(Direction::Up),
                    }
                }
            }
            _ => {
                matrix[npv.y][npv.x] = matrix[gp.y][gp.x];
                matrix[gp.y][gp.x] = Cell::Visited;
                gp = npv;
            }
        }
        np = next_point(&matrix, gp);
    }
    matrix[gp.y][gp.x] = Cell::Visited;

    matrix
        .iter()
        .flat_map(|row| row.iter())
        .filter(|cell| matches!(*cell, Cell::Visited))
        .count()
}

fn get_matrix(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '#' => Cell::Obstruction,
                    '^' => Cell::Guard(Direction::Up),
                    '>' => Cell::Guard(Direction::Right),
                    'v' => Cell::Guard(Direction::Down),
                    '<' => Cell::Guard(Direction::Left),
                    _ => Cell::Empty,
                })
                .collect::<Vec<Cell>>()
        })
        .filter(|row| !row.is_empty())
        .collect::<Vec<Vec<Cell>>>()
}

fn next_point(matrix: &[Vec<Cell>], point: Point) -> Option<Point> {
    let mut new = point;
    if let Cell::Guard(dir) = &matrix[point.y][point.x] {
        match *dir {
            Direction::Up => match new.y.checked_sub(1) {
                Some(y) => new.y = y,
                None => return None,
            },
            Direction::Right => match new.x.checked_add(1) {
                Some(x) => new.x = x,
                None => return None,
            },
            Direction::Down => match new.y.checked_add(1) {
                Some(y) => new.y = y,
                None => return None,
            },
            Direction::Left => match new.x.checked_sub(1) {
                Some(x) => new.x = x,
                None => return None,
            },
        }
    }
    if new.y >= matrix.len() || new.x >= matrix[new.y].len() {
        None
    } else {
        Some(new)
    }
}

fn guard_point(matrix: &[Vec<Cell>]) -> Option<Point> {
    for (y, row) in matrix.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            match cell {
                Cell::Guard(_) => {
                    return Some(Point { x, y });
                }
                _ => continue,
            }
        }
    }
    None
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
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
            ";

    #[test]
    fn example1() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 41);
    }

    #[test]
    fn answer1() {
        let input = fs::read_to_string("./src/day6/input").expect("read input");
        let result = part1(&input);
        print!("answer1 {}", result);
    }

    #[test]
    fn example2() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 0);
    }

    #[test]
    fn answer2() {
        let input = fs::read_to_string("./src/day6/input").expect("read input");
        let result = part2(&input);
        print!("answer2 {}", result);
    }
}
