extern crate num_traits;

use num_traits::pow;

fn solve() {
    // why 360_000?
    // since 360_000 ~ 9**5 * k < 99...9, when k=6
    let N: usize = 360_000;
    let mut res = 0;
    for i in 2..N {
        if is_valid(i) {
            println!("{}", i);
            res += i;
        }
    }
    println!("{}", res);
}

fn is_valid(n: usize) -> bool {
    let mut x = n;
    let mut sp = 0;
    while x != 0 {
        sp += pow((x % 10), 5);
        x /= 10;
    }
    sp == n
}

pub fn main() {
    solve();
}
