extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::mem::replace;

// Calculate large fibonacci numbers.
fn fib(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for i in 0..n {
        let f2 = f0 + &f1;
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
        let bits = f0.bits();
        let log2_10 = (10.0f64).log2();
        // 3.321928094887362
        //println!("{}", log2_10);
        let lim = log2_10 * 999f64;
        //3318.6061667924746
        //println!("{}", lim);
        if bits >= 3318 && bits <= 3322 {
            println!("{:?}", (i + 1, bits));
        }
        // note .bits(), are express needed;
        // log2_F >= 3318.6, i.e. F>= 2^3318.6, need 3319 bits;
        // so test from >=3319
        // get the answer: 4782;
    }
    f0
}

pub fn main() {
    fib(5000);
}
