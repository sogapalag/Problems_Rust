extern crate primal;

use primal::Primes;

fn prime_sum_till(n: usize) -> usize {
    let pset = Primes::all();
    pset.take_while(|&p| p < n).sum()
}

fn solve() {
    let n: usize = 2_000_000;
    let res = prime_sum_till(n);
    println!("{}", res);
}

pub fn main() {
    solve();
}
