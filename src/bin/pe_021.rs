extern crate primal;

use num_traits::pow;
use primal::{Primes, Sieve};

// d(d(n)) == n && d(n)!=n
fn solve(LIMIT: usize) -> usize {
    let si = Sieve::new(LIMIT);
    let mut sum: usize = 0;
    for n in 1..=LIMIT {
        let d = amid(&si, n) - n;
        if d != n && amid(&si, d) - d == n {
            sum += d;
        }
    }
    sum
}

//prod (1+p+p^2+..+p^e)
fn amid(si: &Sieve, n: usize) -> usize {
    match si.factor(n) {
        Ok(fac) => fac
            .iter()
            .map(|x| (pow(x.0, x.1 + 1) - 1) / (x.0 - 1))
            .fold(1, |acc, i| acc * i),
        Err(_) => 1,
    }
}

pub fn main() {
    let res = solve(10000);
    println!("{}", res);
}
