extern crate itertools;
extern crate primal;

use num_traits::pow;
use primal::{Primes, Sieve};

use itertools::Itertools;

fn solve() -> usize {
    let si = Sieve::new(50000);
    // aboudant numbers
    let limit: usize = 28123;
    let abn: Vec<usize> = (1..limit).filter(|&x| amid(&si, x) > x).collect();
    println!("{}", abn.len());
    println!("{:?}", (abn[0], abn[abn.len() - 1]));
    let other_abn: Vec<usize> = (1..limit).filter(|&x| amid(&si, x) > x).collect();
    //println!("{:?}", abn);
    let res = abn
        .iter()
        .cartesian_product(other_abn.iter())
        .map(|c| c.0 + c.1)
        .unique()
        .filter(|&x| x < limit)
        .fold(0, |acc, i| acc + i);
    println!("{}", res);
    limit * (limit - 1) / 2 - res
}

// not same pe_021.rs, already -n
fn amid(si: &Sieve, n: usize) -> usize {
    match si.factor(n) {
        Ok(fac) => {
            fac.iter()
                .map(|x| (pow(x.0, x.1 + 1) - 1) / (x.0 - 1))
                .fold(1, |acc, i| acc * i)
                - n
        }
        Err(_) => 1,
    }
}

// about 10~20s?
pub fn main() {
    let res = solve();
    println!("{}", res);
}
