extern crate num_integer;

use num_integer as ni;

pub fn main() {
    let res = ni::binomial(40 as usize, 20 as usize);
    //137846528820
    println!("{:?}", res);
}
