use std::ops::Mul;

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

fn is_safe(a: isize, b: isize, sign: isize) -> bool {
    (1..=3).contains(&(b - a).mul(sign))
}

fn is_report_safe(report: &[isize], sign: isize) -> bool {
    for (prev, curr) in report.iter().zip(report.iter().skip(1)) {
        if !is_safe(*prev, *curr, sign) {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
fn part1(input: &str) -> usize {
    get_reports(input)
        .iter()
        .filter(|&report| {
            let sign = (report[1] - report[0]).signum() | 1;
            is_report_safe(report, sign)
        })
        .count()
}

#[allow(dead_code)]
fn part2(input: &str) -> usize {
    get_reports(input)
        .iter()
        .filter(|&report| is_report_safe_brute_force(report))
        //  .filter(|report| is_report_safe3(report, 1) || is_report_safe3(report, -1))
        .count()
}

fn is_report_safe_brute_force(report: &Vec<isize>) -> bool {
    if is_report_safe(report, 1) || is_report_safe(report, -1) {
        return true;
    }
    for i in 0..report.len() {
        let mut x = report.clone();
        x.remove(i);
        if is_report_safe(&x, 1) || is_report_safe(&x, -1) {
            return true;
        }
    }
    false
}

#[allow(dead_code)]
fn is_report_safe3(report: &[isize], sign: isize) -> bool {
    let mut change = 0;
    for w in report.windows(3) {
        let safe1 = if change == 1 {
            change = 2;
            true
        } else {
            is_safe(w[0], w[1], sign)
        };
        let safe2 = is_safe(w[1], w[2], sign);
        if safe1 && safe2 {
            continue;
        }
        if !safe1 && safe2 {
            change = 2;
            continue;
        }
        if safe1 && !safe2 {
            change = 1;
            continue;
        }

        if change > 0 || !is_safe(w[0], w[2], sign) {
            return false;
        }
        change = 1;
    }
    true
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    const EXAMPLE: &str = "
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
            ";

    #[test]
    fn example1() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 2);
    }

    #[test]
    fn answer1() {
        let input = fs::read_to_string("./src/day2/input").expect("read input");
        let result = part1(&input);
        print!("answer1 {}", result);
        assert_eq!(result, 236);
    }

    #[test]
    fn example2() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 4);
    }

    #[test]
    fn answer2() {
        let input = fs::read_to_string("./src/day2/input").expect("read input");
        let result = part2(&input);
        assert!(result < 327);
        assert!(result > 261);
        assert_ne!(result, 265);
        assert_ne!(result, 266);
        assert_ne!(result, 277);
        print!("answer2 {}", result);
        assert_eq!(result, 308);
    }
}
