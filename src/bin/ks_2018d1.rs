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

#[macro_export]
macro_rules! cin {
    () => {};
    ($e0:expr) => {
        let stdin = std::io::stdin();
        $e0 = String::from_utf8(
            std::io::Read::bytes(stdin.lock())
                .map(|c| c.unwrap())
                .skip_while(|c| c.is_ascii_whitespace())
                .take_while(|c| !c.is_ascii_whitespace())
                .collect())
            .expect("string")
            .parse().expect("valid parse");
    };
    ($e0:expr, $($ei:tt)*) => {
        let stdin = std::io::stdin();
        $e0 = String::from_utf8(
            std::io::Read::bytes(stdin.lock())
                .map(|c| c.unwrap())
                .skip_while(|c| c.is_ascii_whitespace())
                .take_while(|c| !c.is_ascii_whitespace())
                .collect())
            .expect("string")
            .parse().expect("valid parse");
        cin!($($ei)*);
    };
}

use std::cmp;
use std::collections::{BTreeMap, BTreeSet};
use std::collections::{HashMap, HashSet};
use std::{mem, ptr};

type vi32 = Vec<i32>;
type vi64 = Vec<i64>;
type vu32 = Vec<u32>;
type vu64 = Vec<u64>;
type vusz = Vec<usize>;
type vf32 = Vec<f32>;
type vf64 = Vec<f64>;

const MOD: u64 = 1_000_000_007;
const NINF: i64 = i64::min_value();

fn solve() {
    let n: usize;
    let o: u32;
    let d: i64;
    let x1: i64;
    let x2: i64;
    let a: i64;
    let b: i64;
    let c: i64;
    let m: i64;
    let l: i64;
    cin!(n, o, d);
    cin!(x1, x2, a, b, c, m, l);
    //println!("{:?}", (x1, x2, a, b, c, m, l));

    //create candies
    let mut x = vec![0i64; n + 1];
    let mut s = vec![0i64; n + 1];
    x[1] = x1;
    x[2] = x2;
    for i in 3..n + 1 {
        x[i] = (a * x[i - 1] + b * x[i - 2] + c) % m;
    }
    for i in 1..n + 1 {
        s[i] = x[i] + l;
    }
    //println!("s{:?}", s);

    let mut cumswt = vec![0i64; n + 1];
    let mut cumodd = vec![0u32; n + 1];
    for i in 1..n + 1 {
        cumswt[i] = cumswt[i - 1] + s[i];
        cumodd[i] = cumodd[i - 1];
        if s[i] & 1 != 0 {
            cumodd[i] += 1;
        }
    }
    //println!("cumswt{:?}", cumswt);
    //println!("cumodd{:?}", cumodd);

    let mut res = NINF;
    let mut bt = BTreeMap::new();
    let mut j: usize = 1;
    for i in 1..n + 1 {
        if j < i {
            j = i;
        }
        while j <= n {
            if cumodd[j] - cumodd[i - 1] <= o {
                *bt.entry(cumswt[j]).or_insert(0) += 1;
                j += 1;
            } else {
                break;
            }
        }
        let target = cumswt[i - 1] + d;
        if let Some((&k, _)) = bt.range(..target + 1).rev().next() {
            res = cmp::max(res, k - cumswt[i - 1]);
        }
        let mut c = 1;
        if let Some(x) = bt.get_mut(&cumswt[i]) {
            *x -= 1;
            c = *x;
        }
        if c == 0 {
            bt.remove(&cumswt[i]);
        }
    }
    if res != NINF {
        println!("{}", res);
    } else {
        println!("IMPOSSIBLE");
    }
}

pub fn main() {
    let T = read!(usize);
    for t in 1..T + 1 {
        print!("Case #{}: ", t);
        solve();
    }
}
