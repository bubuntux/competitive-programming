fn get_reports(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .filter(|report| !report.is_empty())
        .collect::<Vec<Vec<isize>>>()
}

#[allow(dead_code)]
fn part1(input: &str) -> usize {
    get_reports(input)
        .iter()
        .fold(0, |mut acc, report| -> usize {
            if is_safe(report) {
                acc += 1
            }
            acc
        })
}

fn is_safe(report: &[isize]) -> bool {
    let mut direction: Option<bool> = None;
    for (prev, curr) in report.iter().zip(report.iter().skip(1)) {
        let mut curr_dir = false;
        let diff = prev - curr;
        if diff == 0 {
            return false;
        }
        if diff > 0 {
            curr_dir = true;
        }
        match direction {
            None => direction = Some(curr_dir),
            Some(prev_dir) => {
                if prev_dir != curr_dir {
                    return false;
                }
            }
        }

        let abs_diff = prev.abs_diff(*curr);
        if !(1..=3).contains(&abs_diff) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    #[test]
    fn example1() {
        let result = part1(
            "
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
            ",
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn answer1() {
        let input = fs::read_to_string("./src/day2/input").expect("read input");
        let result = part1(&input);
        print!("answer1 {}", result);
    }
}
