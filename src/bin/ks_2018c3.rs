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
macro_rules! unpack {
    ($v:expr, $i:expr) => (
        if $i <= 0{panic!("can't unpack <=0 items.")}
        match $i {
            1 => v[0],
            i => unpack!($v, i-1), v[i-1],
        }
    )
}

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

const MOD: u64 = 1_000_000_007;
const LOG: usize = 36; //log2 MOD

fn solve() {
    let mut v: Vec<_>;
    read!(u64, v);
    let (n, k, x1, y1, c, d, e1, e2, f) = (v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8]);
    //println!("{:?}", (n, k, x1, y1, c, d, e1, e2, f));
    let n = n as usize;
    let mut x = vec![0u64; n + 1];
    let mut y = vec![0u64; n + 1];
    let mut A = vec![0u64; n + 1];
    x[1] = x1;
    y[1] = y1;
    for i in 1..n {
        x[i + 1] = (c * x[i] + d * y[i] + e1) % f;
        y[i + 1] = (d * x[i] + c * y[i] + e2) % f;
    }
    for i in 1..n + 1 {
        A[i] = (x[i] + y[i]) % f;
        // note after is MOD
        A[i] = ((n - i + 1) as u64 * A[i]) % MOD;
    }
    //println!("{:?}", A);

    // IE[i][e] = i^(2^e)
    let mut IE = vec![vec![0u64; LOG]; n + 1];
    for i in 1..n + 1 {
        IE[i][0] = i as u64 % MOD;
        for e in 1..LOG {
            IE[i][e] = (IE[i][e - 1] * IE[i][e - 1]) % MOD;
        }
    }
    //println!("{:?}", IE);
    // (i^1+..+i^k) = i(i^k-1)/(i-1);
    // init be 1, careful not zero. cause we first do *
    let mut MK = vec![1u64; n + 1];
    MK[1] = k;
    // x
    let mut kk = k + 1;
    let mut e = 0;
    while kk != 0 {
        if kk & 1 != 0 {
            for i in 2..n + 1 {
                MK[i] = (MK[i] * IE[i][e]) % MOD;
            }
        }
        kk >>= 1;
        e += 1;
    }
    for i in 2..n + 1 {
        MK[i] = (MOD + MK[i] - IE[i][0]) % MOD;
    }
    //  1/(i-1) euler's theorem
    kk = MOD - 2;
    e = 0;
    while kk != 0 {
        if kk & 1 != 0 {
            for i in 2..n + 1 {
                MK[i] = (MK[i] * IE[i - 1][e]) % MOD;
            }
        }
        kk >>= 1;
        e += 1;
    }
    //println!("{:?}", MK);
    let mut res = 0;
    (1..n + 1).rev().fold(0u64, |acc, i| {
        let fac = (acc + A[i]) % MOD;
        res = (res + fac * MK[i]) % MOD;
        fac
    });
    println!("{}", res);
}

pub fn main() {
    let T = read!(usize);
    for t in 1..T + 1 {
        print!("Case #{}: ", t);
        solve();
    }
}
