//0.123456789101112131415161718192021...

extern crate num_traits;

use num_traits::pow;

fn solve() {
    // for i in 1..100{
    //     print!("{}", d(i));
    // }
    let res = (0..7).map(|i| d(pow(10, i))).fold(1, |acc, i| acc * i);
    println!("{}", res);
    // 210
}

fn d(n: u32) -> u32 {
    let mut p = 9;
    let mut k = 1;
    let mut x = n;
    while x > p * k {
        x -= p * k;
        p *= 10;
        k += 1;
    }
    x -= 1;
    let mut a = x / k + p / 9;
    let b = x % k + 1;
    while k != b {
        a /= 10;
        k -= 1;
    }
    a % 10
}

pub fn main() {
    solve();
}
