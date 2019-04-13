// all read_line
// default i32
// let t = read!()
// let t = read!(f64)
// read a Vec<type> into t
// read!(type, t)
#[macro_export]
macro_rules! read {
    () => {{
        let mut buffer = std::string::String::new();
        let stdin = std::io::stdin();
        match stdin.read_line(&mut buffer) {
            Err(err) => panic!("{:?}", err),
            Ok(_) => buffer.trim().parse::<i32>().unwrap(),
        }
    }};
    ($t:ty) => {{
        let mut buffer = std::string::String::new();
        let stdin = std::io::stdin();
        match stdin.read_line(&mut buffer) {
            Err(err) => panic!("{:?}", err),
            Ok(_) => buffer.trim().parse::<$t>().unwrap(),
        }
    }};
    ($t:ty, $e:expr) => {{
        let mut buffer = std::string::String::new();
        let stdin = std::io::stdin();
        match stdin.read_line(&mut buffer) {
            Err(err) => panic!("{:?}", err),
            Ok(_) => {
                $e = buffer
                    .split_whitespace()
                    .map(|s| s.parse::<$t>().unwrap())
                    .collect()
            }
        }
    }};
}

use std::cmp;
use std::collections::{HashMap, HashSet};

// compute under <=n, n could not be valid FL.
fn id(n: &String) -> u64 {
    let digits: Vec<u64> = n.chars().map(|d| d.to_digit(10).unwrap() as u64).collect();
    let l = digits.len();
    let mut res = 0u64;
    let mut no9 = true;
    // count under d00..0
    for i in 0..l - 1 {
        let mut d = digits[i];
        let exp = (l - i - 2) as u32;
        if d != 9 {
            res += d * 9u64.pow(exp) * 8;
        } else {
            d -= 1;
            res += d * 9u64.pow(exp) * 8;
            no9 = false;
            break;
        }
    }
    // count last
    if no9 {
        let mut m = 0;
        for i in 0..l - 1 {
            m += digits[i];
        }
        m = (9 - m % 9) % 9;
        let d = digits[l - 1];
        res += d + 1;
        if m <= d {
            res -= 1;
        }
        if d == 9 {
            res -= 1;
        }
    }
    res
}

fn solve(t: usize) {
    let mut fl: Vec<_>;
    read!(String, fl);
    let res = id(&fl[1]) - id(&fl[0]) + 1;
    println!("Case #{}: {}", t, res);
}

pub fn main() {
    let T = read!(usize);
    for t in (1..T + 1) {
        solve(t);
    }
}
