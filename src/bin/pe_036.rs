// one can brute force 10^6
// or, 1~999 leading, 10^3

extern crate num_traits;

use num_traits::pow;
use std::time::{Duration, Instant};

fn solve() {
    let mut sum = 0;
    for i in 1..=999 {
        let (t0, t1) = create(i);
        if is_p2(t0) {
            sum += t0;
            //println!("{}", t0);
        }
        if is_p2(t1) {
            sum += t1;
            //println!("{}", t1);
        }
    }
    println!("{}", sum);
}

fn create(n: u32) -> (u32, u32) {
    let mut x = n;
    let mut k = 0;
    let mut y = 0; // rev n
    while x != 0 {
        let d = x % 10;
        y *= 10;
        y += d;
        k += 1;
        x /= 10;
    }
    ((n / 10) * pow(10, k) + y, n * pow(10, k) + y)
}

fn is_p2(t: u32) -> bool {
    let s = format!("{:b}", t);
    s == s.chars().rev().collect::<String>()
}

fn naive_solve() {
    let mut sum = 0;
    for i in 1..1_000_000 {
        if is_p(format!("{}", i)) && is_p2(i) {
            sum += i;
        }
    }
    println!("naive, {}", sum);
}

fn is_p(s: String) -> bool {
    s == s.chars().rev().collect::<String>()
}

pub fn main() {
    let start = Instant::now();
    solve();
    //872187
    let duration = start.elapsed();
    let n_start = Instant::now();
    naive_solve();
    let n_duration = n_start.elapsed();
    println!("{:?}, {:?}", duration, n_duration);
    // 10.0ms, 2.38s
}
