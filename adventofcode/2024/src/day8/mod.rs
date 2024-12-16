use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Direction {
    UR,
    UL,
    DR,
    DL,
}

fn get_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[allow(dead_code)]
fn part1(input: &str) -> usize {
    let matrix = get_matrix(input);
    let map = get_map(&matrix);

    let directions = vec![Direction::UR, Direction::UL, Direction::DR, Direction::DL];

    let mut antinodes: HashSet<Point> = HashSet::new();
    for points in map.values() {
        let points = points.iter().collect::<Vec<&Point>>();
        for (i, p1) in points.iter().take(points.len() - 1).enumerate() {
            for p2 in points.iter().skip(i + 1).take(points.len()) {
                let (p1, p2) = (*p1, *p2);
                let (x_distance, y_distance, slope) = get_metadata(p1, p2);
                for dir in &directions {
                    let from = get_edge_node(dir, p1, p2);
                    if let Some(antinode) =
                        get_antinode(&matrix, slope, &from, x_distance, y_distance, dir)
                    {
                        antinodes.insert(antinode);
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn get_edge_node(dir: &Direction, p1: &Point, p2: &Point) -> Point {
    match dir {
        Direction::UR => Point {
            x: p1.x.max(p2.x),
            y: p1.y.min(p2.y),
        },
        Direction::UL => Point {
            x: p1.x.min(p2.x),
            y: p1.y.min(p2.y),
        },
        Direction::DR => Point {
            x: p1.x.max(p2.x),
            y: p1.y.max(p2.y),
        },
        Direction::DL => Point {
            x: p1.x.min(p2.x),
            y: p1.y.max(p2.y),
        },
    }
}

fn get_metadata(p1: &Point, p2: &Point) -> (usize, usize, bool) {
    let x_distance = p1.x.abs_diff(p2.x);
    let y_distance = p1.y.abs_diff(p2.y);

    let ((_, sx), (_, sy)) = (p2.x.overflowing_sub(p1.x), p2.y.overflowing_sub(p1.y));
    let slope = sx == sy;
    (x_distance, y_distance, slope)
}

fn get_map(matrix: &[Vec<char>]) -> HashMap<char, HashSet<Point>> {
    let mut map: HashMap<char, HashSet<Point>> = HashMap::new();
    for (y, row) in matrix.iter().enumerate() {
        for (x, node) in row.iter().enumerate() {
            if node != &'.' {
                map.entry(*node).or_default().insert(Point { x, y });
            }
        }
    }
    map
}

fn get_antinode(
    matrix: &[Vec<char>],
    slope: bool,
    from: &Point,
    x_distance: usize,
    y_distance: usize,
    dir: &Direction,
) -> Option<Point> {
    let (antinode, overflow) = match dir {
        Direction::UR if !slope => {
            let x = from.x + x_distance;
            let (y, overflow) = from.y.overflowing_sub(y_distance);
            (Point { x, y }, overflow)
        }
        Direction::UL if slope => {
            let (x, x_overflow) = from.x.overflowing_sub(x_distance);
            let (y, y_overflow) = from.y.overflowing_sub(y_distance);
            (Point { x, y }, (x_overflow || y_overflow))
        }
        Direction::DR if slope => {
            let x = from.x + x_distance;
            let y = from.y + y_distance;
            (Point { x, y }, false)
        }
        Direction::DL if !slope => {
            let (x, overflow) = from.x.overflowing_sub(x_distance);
            let y = from.y + y_distance;
            (Point { x, y }, overflow)
        }
        _ => (*from, true),
    };

    if overflow || antinode.y >= matrix.len() || antinode.x >= matrix[antinode.y].len() {
        None
    } else {
        Some(antinode)
    }
}

#[allow(dead_code)]
fn part2(input: &str) -> usize {
    let matrix = get_matrix(input);
    let map = get_map(&matrix);

    let directions = vec![Direction::UR, Direction::UL, Direction::DR, Direction::DL];

    let mut antinodes: HashSet<Point> = HashSet::new();
    for points in map.values() {
        let points = points.iter().collect::<Vec<&Point>>();
        for (i, p1) in points.iter().take(points.len() - 1).enumerate() {
            for p2 in points.iter().skip(i + 1).take(points.len()) {
                let (p1, p2) = (*p1, *p2);
                let (x_distance, y_distance, slope) = get_metadata(p1, p2);
                // println!(
                //     "{:?},{:?},{:?},{:?},{:?}",
                //     p1, p2, slope, x_distance, y_distance
                // );
                antinodes.insert(*p1);
                antinodes.insert(*p2);
                for dir in &directions {
                    let mut from = get_edge_node(dir, p1, p2);
                    while let Some(antinode) =
                        get_antinode(&matrix, slope, &from, x_distance, y_distance, dir)
                    {
                        //println!("{:?},{:?},{:?}", dir, from, antinode);
                        antinodes.insert(antinode);
                        from = antinode;
                    }
                }
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    const EXAMPLE: &str = "
            ............
            ........0...
            .....0......
            .......0....
            ....0.......
            ......A.....
            ............
            ............
            ........A...
            .........A..
            ............
            ............
            ";

    #[test]
    fn example1() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 14);
    }

    #[test]
    fn answer1() {
        let input = fs::read_to_string("./src/day8/input").expect("read input");
        let result = part1(&input);
        print!("answer1 {}", result);
        assert_eq!(result, 247);
    }

    #[test]
    fn example2_0() {
        let result = part2(
            "
            T.........
            ...T......
            .T........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            ",
        );
        assert_eq!(result, 9);
    }

    #[test]
    fn example2() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 34);
    }

    #[test]
    fn answer2() {
        let input = fs::read_to_string("./src/day8/input").expect("read input");
        let result = part2(&input);
        print!("answer2 {}", result);
        assert!(result > 858);
        assert_eq!(result, 861);
    }
}
