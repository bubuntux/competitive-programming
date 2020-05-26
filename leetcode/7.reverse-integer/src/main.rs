fn main() {}

pub fn reverse(mut x: i32) -> i32 {
    let mut rev = 0;

    while x != 0 {
        let pop = x % 10;
        x /= 10;
        let max = std::i32::MAX / 10;
        if rev > max || (rev == max && pop > 7) {
            return 0;
        }
        let min = std::i32::MIN / 10;
        if rev < min || (rev == min && pop < -8) {
            return 0;
        }

        rev = rev * 10 + pop;
    }

    rev
}

#[test]
fn tests() {
    assert_eq!(reverse(123), 321);
    assert_eq!(reverse(120), 21);
    assert_eq!(reverse(-123), -321);
}
