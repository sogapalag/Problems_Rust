extern crate primal;

use primal::{Primes, Sieve};

// find triangle number with more than k div.(factors)
fn tri_num_div_over(k: usize) -> usize {
    let si = Sieve::new(100_000_000);
    for i in 1..15000{
        let tri:usize = i*(i+1)/2;
        let x = num_of_divisors(&si, tri); 
        println!("{:?}", (tri, x));
        if x >= k {
            return tri;
        }
    }
    0
    
}

// test use
fn num_of_divisors(si: & Sieve, n: usize) -> usize{
    //let ps = Primes::all();
    //let si = Sieve::new(200);
    //let i:i32 = si.factor(n);
    match si.factor(n){
        Ok(fac) => fac.iter().map(|x| x.1+1).fold(1, |acc, i| acc*i),
        Err(_) => 0,
    }
    //let t = Primes::is_prime(7);
    //0
}

pub fn main() {
    let t = tri_num_div_over(500);
    println!("{}", t);
}
