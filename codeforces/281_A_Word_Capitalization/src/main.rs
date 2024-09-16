// https://codeforces.com/problemset/problem/281/A

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {mut input: String}

    print!("{}", input)
}


mod sub {
    // You can also `use` the crate in submodules.

    #[allow(unused_imports)]
    use proconio::input as _;
}