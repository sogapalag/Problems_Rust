// all read_line
// default i64
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
            Ok(_) => buffer.trim().parse::<i64>().unwrap(),
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
use std::collections::{HashMap, HashSet};
use std::collections::{BTreeMap, BTreeSet};
use std::{mem, ptr};

type vi32 = Vec<i32>;
type vi64 = Vec<i64>;
type vu32 = Vec<u32>;
type vu64 = Vec<u64>;
type vusz = Vec<usize>;
type vf32 = Vec<f32>;
type vf64 = Vec<f64>;

type v2i64 = Vec<[i64;2]>;

const MOD: u64 = 1_000_000_007;
const NINF: i64 = i64::min_value();

fn solve() {
    let n: usize;
    let k: usize;
    let mut vp: vi64;
    let mut vh: vi64;
    let mut vx: vi64;
    let mut vy: vi64;
    cin!(n, k);
    read!(i64, vp);
    read!(i64, vh);
    read!(i64, vx);
    read!(i64, vy);
  //  let mut p = vec![0i64; n + 1];
  //  let mut h = vec![0i64; n + 1];
  //  let mut x = vec![0i64; n + 1];
  //  let mut y = vec![0i64; n + 1];
  //  p[1] = vp[0]; p[2] = vp[1];
  //  h[1] = vh[0]; h[2] = vh[1];
  //  x[1] = vx[0]; x[2] = vx[1];
  //  y[1] = vy[0]; y[2] = vy[1];
    let mut t = vec![[0i64;2]; n + 1];
    let mut b = vec![[0i64;2]; k + 1];
    t[1] = [vp[0], vh[0]];
    t[2] = [vp[1], vh[1]];
    b[1] = [vx[0], vy[0]];
    b[2] = [vx[1], vy[1]];
    for i in 3..n+1 {
        t[i][0] = (vp[2] * t[i-1][0] + vp[3] * t[i-2][0] + vp[4]) % vp[5] + 1;
        t[i][1] = (vh[2] * t[i-1][1] + vh[3] * t[i-2][1] + vh[4]) % vh[5] + 1;
    }
    for i in 3..k+1 {
        b[i][0] = (vx[2] * b[i-1][0] + vx[3] * b[i-2][0] + vx[4]) % vx[5] + 1;
        b[i][1] = (vy[2] * b[i-1][1] + vy[3] * b[i-2][1] + vy[4]) % vy[5] + 1;
    }
    //println!("{:?}", t);
    //println!("{:?}", b);
    
    t.sort(); b.sort();

    let mut ok = vec![false; k+1];
    let mut res = 0;
    // left -> right
    let mut j = 1;
    let mut PH = [0, 0];
    let cover = |_t: &[i64], _b: &[i64]| {_t[1] - _b[1] >= _b[0] - _t[0] && _b[0] >= _t[0]};
    for i in 1..k+1 {
        while j <= n {
            if t[j][0] <= b[i][0]{
                if !cover(&PH, &t[j]) {
                    PH = t[j];
                }
                j += 1;
            }else{
                break;
            }
        }
        if cover(&PH, &b[i]) {
            ok[i] = true;
            res += 1;
        }
    }
    // left <- right
    j = n;
    PH = [1_000_000_123, 0];
    let cover = |_t: &[i64], _b: &[i64]| {_t[1] - _b[1] >= _t[0] - _b[0] && _t[0] >= _b[0]};
    for i in (1..k+1).rev() {
        while j >= 1 {
            if t[j][0] >= b[i][0] {
                if !cover(&PH, &t[j]) {
                    PH = t[j];
                }
                j -= 1;
            } else{
                break;
            }
        }
        if !ok[i] && cover(&PH, &b[i]){
            ok[i] = true;
            res += 1;
        }
    }
    
    println!("{}", res);
}

pub fn main() {
    let T = read!(usize);
    for t in 1..T + 1 {
        print!("Case #{}: ", t);
        solve();
    }
}
