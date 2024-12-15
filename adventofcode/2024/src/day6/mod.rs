use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Guard {
    point: Point,
    dir: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Obstruction,
    Guard(Direction),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[allow(dead_code)]
fn part1(input: &str) -> usize {
    let matrix = get_matrix(input);

    let g = get_guard(&matrix);
    if g.is_none() {
        return 0;
    }
    let guard = g.unwrap();
    walk(&matrix, guard).unwrap().len()
}

fn walk(matrix: &[Vec<Cell>], guard: Guard) -> Option<HashMap<Point, Direction>> {
    let mut gp = guard.point;
    let mut gd = guard.dir;
    let mut np = next_point(matrix, gp, gd);

    let mut visited = HashMap::new();
    while let Some(npv) = np {
        match matrix[npv.y][npv.x] {
            Cell::Obstruction => {
                gd = match gd {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                }
            }
            _ => {
                if let Some(x) = visited.get(&gp) {
                    if x == &gd {
                        return None;
                    }
                }
                visited.insert(gp, gd);
                gp = npv;
            }
        }
        np = next_point(matrix, gp, gd);
    }
    if let Some(x) = visited.get(&gp) {
        if x == &gd {
            return None;
        }
    }
    visited.insert(gp, gd);

    Some(visited)
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

fn next_point(matrix: &[Vec<Cell>], point: Point, dir: Direction) -> Option<Point> {
    let (new, overflow): (Point, bool) = match dir {
        Direction::Up => {
            let (y, of) = point.y.overflowing_sub(1);
            (Point { x: point.x, y }, of)
        }
        Direction::Right => {
            let (x, of) = point.x.overflowing_add(1);
            (Point { x, y: point.y }, of)
        }
        Direction::Down => {
            let (y, of) = point.y.overflowing_add(1);
            (Point { x: point.x, y }, of)
        }
        Direction::Left => {
            let (x, of) = point.x.overflowing_sub(1);
            (Point { x, y: point.y }, of)
        }
    };
    if overflow || new.y >= matrix.len() || new.x >= matrix[new.y].len() {
        None
    } else {
        Some(new)
    }
}

fn get_guard(matrix: &[Vec<Cell>]) -> Option<Guard> {
    for (y, row) in matrix.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            match cell {
                Cell::Guard(dir) => {
                    return Some(Guard {
                        point: Point { x, y },
                        dir: *dir,
                    })
                }
                _ => continue,
            }
        }
    }
    None
}

#[allow(dead_code)]
fn part2(input: &str) -> usize {
    let mut matrix = get_matrix(input);

    let gp = get_guard(&matrix);
    if gp.is_none() {
        return 0;
    }
    let guard_point = gp.unwrap();
    walk(&matrix, guard_point)
        .unwrap()
        .iter()
        .filter(|(ponit, _)| *ponit != &guard_point.point)
        .filter(|(point, _)| {
            matrix[point.y][point.x] = Cell::Obstruction;
            let visited = walk(&matrix, guard_point);
            matrix[point.y][point.x] = Cell::Empty;
            visited.is_none()
        })
        .count()
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
        assert_eq!(result, 5516);
    }

    #[test]
    fn example2() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 6);
    }

    #[test]
    fn answer2() {
        let input = fs::read_to_string("./src/day6/input").expect("read input");
        let result = part2(&input);
        print!("answer2 {}", result);
        assert_eq!(result, 2008);
    }
}
