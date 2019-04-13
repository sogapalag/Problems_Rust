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
// Even Digits

use std::cmp;

fn compute(N: &String) -> u64 {
    let rem: Vec<_> = N
        .chars()
        .skip_while(|c| c.to_digit(10).unwrap() & 1 == 0)
        .collect();
    if rem.is_empty() {
        return 0;
    }
    let suf = rem.len() - 1;
    let n = rem.iter().collect::<String>().parse::<u64>().unwrap();
    let hi_ = (rem[0] as u8 + 1) as char;
    let lo_ = (rem[0] as u8 - 1) as char;
    let up = format!("{}{}", hi_, "0".repeat(suf));
    let down = format!("{}{}", lo_, "8".repeat(suf));
    let SUB = n - down.parse::<u64>().unwrap();
    match rem[0] {
        '9' => SUB,
        _ => {
            let ADD = up.parse::<u64>().unwrap() - n;
            cmp::min(ADD, SUB)
        }
    }
}

fn solve(t: usize) {
    let N = read!(String);
    println!("Case #{}: {}", t, compute(&N));
}

pub fn main() {
    let T = read!(usize);
    // google rust complier didn't support inclusive 1..=T
    for t in (1..T+1) {
        solve(t);
    }
}
