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

// Fairies and Witches

use std::cmp;
use std::collections::{HashMap, HashSet};
use std::{mem, ptr};

type vi32 = Vec<i32>;
type vi64 = Vec<i64>;
type vu32 = Vec<u32>;
type vu64 = Vec<u64>;
type vusz = Vec<usize>;
type vf32 = Vec<f32>;
type vf64 = Vec<f64>;

#[inline]
fn poly(s: &vu32) -> bool {
    s.len() >= 3 && s.iter().sum::<u32>() > 2 * s.iter().max().unwrap()
}

fn checkfrom(mask: u32, s: &mut vu32, L: &Vec<vu32>, N: usize) -> u32 {
    if mask + 1 == (1 << N) {
        return if poly(s) { 1 } else { 0 };
    }
    let mut res = 0u32;
    for i in 0..N {
        if mask & (1 << i) == 0 {
            // consume node i
            res += checkfrom(mask | (1 << i), s, L, N);
            for j in i + 1..N {
                // consume node i,j with add new edge
                if mask & (1 << j) == 0 && L[i][j] != 0 {
                    s.push(L[i][j]);
                    res += checkfrom(mask | (1 << i) | (1 << j), s, L, N);
                    s.pop();
                }
            }
            // above covered, here must break.
            break;
        }
    }
    res
}

fn solve() {
    let n = read!(usize);
    let mut L: Vec<vu32> = Vec::new();
    for _ in 0..n {
        let mut v: Vec<_>;
        read!(u32, v);
        L.push(v);
    }
    //println!("{:?}", L);

    let mut edges: vu32 = Vec::new();
    let res = checkfrom(0, &mut edges, &L, n);
    println!("{}", res);
}

pub fn main() {
    let T = read!(usize);
    for t in 1..T + 1 {
        print!("Case #{}: ", t);
        solve();
    }
}
