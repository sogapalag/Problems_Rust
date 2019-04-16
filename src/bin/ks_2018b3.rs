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
use std::ops::{AddAssign, Sub};

type _2u64 = (u64, u64);
type _4u64 = (u64, u64, u64, u64);
type v2u64 = Vec<_2u64>;
type v4u64 = Vec<_4u64>;

// update aka. tree[idx], [+LSB]...
pub fn update<T>(fenwick: &mut [T], mut idx: usize, val: T)
where
    T: AddAssign + Copy + Default,
{
    while idx < fenwick.len() {
        fenwick[idx] += val;
        idx = (idx | (idx - 1)) + 1;
    }
}
// sum in [1, idx], aka. tree[idx], [-LSB]...
pub fn query<T>(fenwick: &[T], mut idx: usize) -> T
where
    T: AddAssign + Copy + Default,
{
    let mut sum = T::default();
    while idx > 0 {
        sum += fenwick[idx];
        idx &= idx - 1;
    }
    sum
}

pub fn qr<T>(fenwick: &[T], left: usize, right: usize) -> T
where
    T: AddAssign + Copy + Default + Sub<Output = T>,
{
    query(fenwick, right) - query(fenwick, left - 1)
}

// compute quad-values.
fn quad(p: &v2u64, n: usize, m: usize) -> v4u64 {
    let mut q: v4u64 = vec![(0, 0, 0, 0); n + 1];

    // linear sweep left->right
    let mut bit = vec![0u64; m + 1];
    let mut i = 1;
    while i <= n {
        let mut j = i;
        let I = loop {
            j += 1;
            if j > n || p[i].0 != p[j].0 {
                break j;
            }
        };
        for k in i..I {
            q[k].0 = qr(&bit, 1, p[k].1 as usize - 1);
            q[k].1 = qr(&bit, p[k].1 as usize + 1, m);
        }
        for k in i..I {
            update(&mut bit, p[k].1 as usize, 1);
        }
        i = I;
    }
    // linear sweep left <- right
    let mut bit = vec![0u64; m + 1];
    let mut i = n;
    while i > 0 {
        let mut j = i;
        let I = loop {
            j -= 1;
            if j == 0 || p[i].0 != p[j].0 {
                break j;
            }
        };
        for k in I + 1..i + 1 {
            q[k].2 = qr(&bit, 1, p[k].1 as usize - 1);
            q[k].3 = qr(&bit, p[k].1 as usize + 1, m);
        }
        for k in I + 1..i + 1 {
            update(&mut bit, p[k].1 as usize, 1);
        }
        i = I;
    }
    q
}

fn solve() {
    let mut v: Vec<_>;
    read!(u64, v);
    let (n, v1, h1, a, b, c, d, e, f, m) = (
        v[0] as usize,
        v[1],
        v[2],
        v[3],
        v[4],
        v[5],
        v[6],
        v[7],
        v[8],
        v[9],
    );

    let mut p = vec![(0u64, 0u64); n + 1];
    p[1] = (v1, h1);
    for i in 1..n {
        p[i + 1].0 = (a * p[i].0 + b * p[i].1 + c) % m;
        p[i + 1].1 = (d * p[i].0 + e * p[i].1 + f) % m;
    }
    for i in 1..n + 1 {
        p[i].0 += 1;
        p[i].1 += 1;
    }
    p.sort();

    let q = quad(&p, n, m as usize);

    let n = n as u64;
    let mut res = (n * (n - 1) * (n - 2)) / 6;
    for e in q.iter() {
        res -= e.0 * e.3 + e.1 * e.2;
    }

    println!("{}", res);
}

pub fn main() {
    let T = read!(usize);
    for t in (1..T + 1) {
        print!("Case #{}: ", t);
        solve();
    }
}
