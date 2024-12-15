use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
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

    let mut map: HashMap<char, HashSet<Point>> = HashMap::new();
    for (y, row) in matrix.iter().enumerate() {
        for (x, node) in row.iter().enumerate() {
            if node != &'.' {
                map.entry(*node).or_default().insert(Point { x, y });
            }
        }
    }

    let mut antinodes: HashSet<Point> = HashSet::new();
    for points in map.values() {
        let points = points.iter().collect::<Vec<&Point>>();
        for (i, p1) in points.iter().take(points.len() - 1).enumerate() {
            for p2 in points.iter().skip(i + 1).take(points.len()) {
                let (p1, p2) = (*p1, *p2);
                let x_distance = p1.x.abs_diff(p2.x);
                let y_distance = p1.y.abs_diff(p2.y);

                let ((_, sx), (_, sy)) = (p2.x.overflowing_sub(p1.x), p2.y.overflowing_sub(p1.y));
                let slope = sx == sy;

                if slope {
                    let x1 = p1.x.max(p2.x) + x_distance;
                    let y1 = p1.y.max(p2.y) + y_distance;
                    if y1 < matrix.len() && x1 < matrix[y1].len() {
                        let value = Point { x: x1, y: y1 };
                        antinodes.insert(value);
                    }

                    let (x2, xo) = p1.x.min(p2.x).overflowing_sub(x_distance);
                    let (y2, yo) = p1.y.min(p2.y).overflowing_sub(y_distance);
                    if !xo && !yo {
                        let value = Point { x: x2, y: y2 };
                        antinodes.insert(value);
                    }
                } else {
                    let x1 = p1.x.max(p2.x) + x_distance;
                    let (y2, yo) = p1.y.min(p2.y).overflowing_sub(y_distance);
                    if !yo && x1 < matrix[y2].len() {
                        let value = Point { x: x1, y: y2 };
                        antinodes.insert(value);
                    }

                    let (x2, xo) = p1.x.min(p2.x).overflowing_sub(x_distance);
                    let y1 = p1.y.max(p2.y) + y_distance;
                    if !xo && y1 < matrix.len() {
                        let value = Point { x: x2, y: y1 };
                        antinodes.insert(value);
                    }
                }
            }
        }
    }

    antinodes.len()
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
    fn example2() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 0);
    }

    #[test]
    fn answer2() {
        let input = fs::read_to_string("./src/day8/input").expect("read input");
        let result = part2(&input);
        print!("answer2 {}", result);
    }
}
