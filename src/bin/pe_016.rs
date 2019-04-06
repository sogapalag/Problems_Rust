extern crate num;
extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigInt;

pub fn main() {
    let base = BigInt::parse_bytes(b"2", 10).unwrap();
    let res: u32 = num_traits::pow(base, 1000 as usize)
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum();
    println!("{:?}", res);
}
