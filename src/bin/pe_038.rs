// one can easily verify
// 1-9, has 1
// 1-8,7,6 impossible
// 1-5 has 9
// 1-4, possible 25<= . <= 33
// 1-3, possible 100<= . <= 333
// 1-2, possible 5000<= . <= 9999
extern crate itertools;

use itertools::Itertools;
use std::cmp;

fn solve() {
    let mut res = 0;
    for i in 25..=33 {
        let n = (1..=4)
            .map(|x| (i * x).to_string())
            .join("")
            .parse::<u32>()
            .unwrap();
        if is_pandigital(n) {
            res = cmp::max(res, n);
            println!("{:?}", (n, i));
        }
    }
    for i in 100..=333 {
        let n = (1..=3)
            .map(|x| (i * x).to_string())
            .join("")
            .parse::<u32>()
            .unwrap();
        if is_pandigital(n) {
            res = cmp::max(res, n);
            println!("{:?}", (n, i));
        }
    }
    for i in 5000..=9999 {
        let n = (1..=2)
            .map(|x| (i * x).to_string())
            .join("")
            .parse::<u32>()
            .unwrap();
        if is_pandigital(n) {
            res = cmp::max(res, n);
            println!("{:?}", (n, i));
        }
    }
    println!("res {}", res);
    //932718654
}
fn is_pandigital(n: u32) -> bool {
    let mut set: u32 = 0;
    let mut x = n;
    while x != 0 {
        let d = x % 10;
        if d == 0 {
            return false;
        }
        if set & (1 << (d - 1)) != 0 {
            return false;
        }
        set |= 1 << (d - 1);
        x /= 10;
    }
    set ^ 0b111_111_111 == 0
}

pub fn main() {
    solve();
}
