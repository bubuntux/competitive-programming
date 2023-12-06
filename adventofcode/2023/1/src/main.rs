use std::fs;

fn main() {
    let content = fs::read_to_string("./src/input").expect("Unable to read the file");
    let result = process(&content);
    println!("result {}", result);
}

fn process(input: &str) -> usize {
    return input
        .lines()
        .map(|line| -> usize {
            let first = get_first_digit(&line);
            let last = get_last_digit(&line);
            (first * 10) + last
        })
        .sum();
}

fn get_first_digit(input: &str) -> usize {
    return input
        .trim()
        .chars()
        .find(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .unwrap() as usize;
}

fn get_last_digit(input: &str) -> usize {
    return input
        .trim()
        .chars()
        .filter(|c| c.is_digit(10))
        .last()
        .map(|c| c.to_digit(10).unwrap())
        .unwrap() as usize;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let result = process(
            "1abc2
             pqr3stu8vwx
             a1b2c3d4e5f
             treb7uchet",
        );
        assert_eq!(result, 142);
    }
}
