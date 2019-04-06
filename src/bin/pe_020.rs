extern crate num;
extern crate num_bigint;
extern crate num_iter;
extern crate num_traits;

use num_bigint::BigUint;

fn solve() -> u32 {
    let st = BigUint::parse_bytes(b"1", 10).unwrap();
    let en = BigUint::parse_bytes(b"101", 10).unwrap();
    num_iter::range::<BigUint>(st, en)
        .fold(num_traits::one::<BigUint>(), |acc, i| acc * i)
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum()
}

pub fn main() {
    let res = solve();
    println!("{}", res);
}
