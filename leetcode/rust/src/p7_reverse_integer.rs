//! ```text
//! # 7. Reverse Integer
//! https://leetcode.com/problems/reverse-integer/
//!
//! Given a 32-bit signed integer, reverse digits of an integer.
//!
//! Example 1:
//!
//!     Input: 123
//!     Output: 321
//!
//! Example 2:
//!
//!     Input: -123
//!     Output: -321
//!
//! Example 3:
//!
//!     Input: 120
//!     Output: 21
//!
//! Note:
//! Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−231,  231 − 1]. For the purpose of this problem, assume that your function returns 0 when the reversed integer overflows.
//! ```

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
