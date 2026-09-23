// https://rustcurious.com/2

// factorial(5) = 1 * 2 * 3 * 4 * 5
//
// The provided factorial function is recursive.
// Your job is to rewrite the function body to be
// iterative, so it does not call itself. Instead
// use a mutable variable (`let mut`) initialised
// to 1, and a for loop that multiplies it by each
// value in the range from 1 to n.
//
// (For a reminder of for-loop syntax look at main below.)

fn factorial(n: u64) -> u64 {
    let mut acc = 1;
    for i in 2..=n {
        acc *= i;
    }
    acc
}

// -------------------------------------------------------
// No need to change anything below this line.

pub fn main() {
    for i in 0..=20 {
        println!("{}! = {}", i, factorial(i));
    }
}

// This is a unit test. In the playground, switch between
// "Run" and "Test" using the "..." button.
#[test]
fn test_factorial() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(10), 3628800);
    assert_eq!(factorial(20), 2432902008176640000);
    // To go higher than factorial(20)
    // try u128 instead of u64.
}
