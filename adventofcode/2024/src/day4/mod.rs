#[allow(dead_code)]
fn part1(input: &str) -> usize {
    let matrix = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .filter(|row| !row.is_empty())
        .collect::<Vec<Vec<char>>>();

    let mut sum = 0;
    let target = "XMAS";

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if right(&matrix, x, y, target) {
                sum += 1
            }
            if right_down(&matrix, x, y, target) {
                sum += 1
            }
            if down(&matrix, x, y, target) {
                sum += 1
            }
            if down_left(&matrix, x, y, target) {
                sum += 1
            }
            if left(&matrix, x, y, target) {
                sum += 1
            }
            if left_up(&matrix, x, y, target) {
                sum += 1
            }
            if up(&matrix, x, y, target) {
                sum += 1
            }
            if up_right(&matrix, x, y, target) {
                sum += 1
            }
        }
    }

    sum
}

fn right(matrix: &[Vec<char>], x: usize, y: usize, target: &str) -> bool {
    match x + target.len() {
        xt if xt > matrix[y].len() => false,
        xt => matrix[y][x..xt].iter().collect::<String>().eq(target),
    }
}

fn right_down(matrix: &[Vec<char>], x: usize, y: usize, target: &str) -> bool {
    match (x + target.len(), y + target.len()) {
        (xt, yt) if yt > matrix.len() || xt > matrix[y].len() => false,
        (_, yt) => matrix[y..yt]
            .iter()
            .fold((String::from(""), x), |(mut word, mut x), col| {
                word.push(col[x]);
                x += 1;
                (word, x)
            })
            .0
            .eq(target),
    }
}

fn down(matrix: &[Vec<char>], x: usize, y: usize, target: &str) -> bool {
    match y + target.len() {
        yt if yt > matrix.len() => false,
        yt => matrix[y..yt]
            .iter()
            .map(|col| col[x])
            .collect::<String>()
            .eq(target),
    }
}
fn down_left(matrix: &[Vec<char>], x: usize, y: usize, target: &str) -> bool {
    match (x.checked_sub(target.len() - 1), y + target.len()) {
        (xt, yt) if yt > matrix.len() || xt.is_none() => false,
        (_, yt) => matrix[y..yt]
            .iter()
            .fold((String::from(""), x as isize), |(mut word, mut x), col| {
                word.push(col[x as usize]);
                x -= 1;
                (word, x)
            })
            .0
            .eq(target),
    }
}

fn left(matrix: &[Vec<char>], x: usize, y: usize, target: &str) -> bool {
    match x.checked_sub(target.len() - 1) {
        None => false,
        Some(xt) => matrix[y][xt..x + 1]
            .iter()
            .rev()
            .collect::<String>()
            .eq(target),
    }
}

fn left_up(matrix: &[Vec<char>], x: usize, y: usize, target: &str) -> bool {
    match (
        x.checked_sub(target.len() - 1),
        y.checked_sub(target.len() - 1),
    ) {
        (Some(_), Some(yt)) => matrix[yt..y + 1]
            .iter()
            .rev()
            .fold((String::from(""), x as isize), |(mut word, mut x), col| {
                word.push(col[x as usize]);
                x -= 1;
                (word, x)
            })
            .0
            .eq(target),
        (_, _) => false,
    }
}

fn up(matrix: &[Vec<char>], x: usize, y: usize, target: &str) -> bool {
    match y.checked_sub(target.len() - 1) {
        None => false,
        Some(yt) => matrix[yt..y + 1]
            .iter()
            .map(|col| col[x])
            .rev()
            .collect::<String>()
            .eq(target),
    }
}

fn up_right(matrix: &[Vec<char>], x: usize, y: usize, target: &str) -> bool {
    match (x + target.len(), y.checked_sub(target.len() - 1)) {
        (xt, yt) if xt > matrix[y].len() || yt.is_none() => false,
        (_, yt) => matrix[yt.unwrap()..y + 1]
            .iter()
            .rev()
            .fold((String::from(""), x), |(mut word, mut x), col| {
                word.push(col[x]);
                x += 1;
                (word, x)
            })
            .0
            .eq(target),
    }
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
    fn example0_1() {
        let result = part1(
            "
            .....X.
            .SAMXM.
            .A...A.
            XMASAMX
            .X.....
            ",
        );
        assert_eq!(result, 4);
    }

    #[test]
    fn example0_2() {
        let result = part1(
            "
            ..X...
            .SAMXM
            .A.MAA
            XMAS.S
            .S....
            ",
        );
        assert_eq!(result, 4);
    }

    #[test]
    fn example1_2() {
        let result = part1(
            "
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
            ",
        );
        assert_eq!(result, 18);
    }

    #[test]
    fn answer1() {
        let input = fs::read_to_string("./src/day4/input").expect("read input");
        let result = part1(&input);
        print!("answer1 {}", result);
    }

    #[test]
    fn example2() {
        let result =
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, 48);
    }

    #[test]
    fn answer2() {
        let input = fs::read_to_string("./src/day4/input").expect("read input");
        let result = part2(&input);
        print!("answer2 {}", result);
    }
}
