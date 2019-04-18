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

fn gcd(x: u64, y: u64) -> u64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

const N: usize = 101;

static mut sum_d : [[[u64;N];N];N] = [[[0u64; N]; N]; N];
static mut sum_r : [[[u64;N];N];N] = [[[0u64; N]; N]; N];
static mut cum_d : [[[u64;N];N];N] = [[[0u64; N]; N]; N];
static mut cum_r : [[[u64;N];N];N] = [[[0u64; N]; N]; N];

// READ src/notes/ks_2018d3.md fisrt!!

unsafe fn solve() {
    let r: usize;
    let c: usize;
    let w: usize;
    cin!(r, c, w);
    let mut mat = vec![vec![' '; c + 1]; r + 1];
    for i in 1..r + 1 {
        let s = read!(String);
        let mut iter = s.chars();
        for j in 1..c + 1 {
            mat[i][j] = iter.next().unwrap();
        }
    }
    //println!("{:?}", mat);

    let mut hm = HashMap::new();
    for _ in 1..w + 1 {
        let s = read!(String);
        *hm.entry(s.clone()).or_insert(0u64) += s.len() as u64;
        let s = s.chars().rev().collect();
        *hm.entry(s).or_insert(0u64) += s.len() as u64;
    }
    //println!("{:?}", hm);

    ptr::write_bytes(sum_d.as_mut_ptr(), 0, N);
    ptr::write_bytes(sum_r.as_mut_ptr(), 0, N);
    ptr::write_bytes(cum_d.as_mut_ptr(), 0, N);
    ptr::write_bytes(cum_r.as_mut_ptr(), 0, N);

    // d [i,j,k]: start at [i,j], ~within[k,j] total length.
    for i in 1..r + 1 {
        for j in 1..c + 1 {
            let mut s = String::with_capacity(N);
            for k in i..r + 1 {
                s.push(mat[k][j]);
                match hm.get(&s) {
                    Some(&v) => sum_d[i][j][k] = sum_d[i][j][k - 1] + v,
                    None => sum_d[i][j][k] = sum_d[i][j][k - 1],
                }
            }
            let mut s = String::with_capacity(N);
            for k in j..c + 1 {
                s.push(mat[i][k]);
                match hm.get(&s) {
                    Some(&v) => sum_r[i][j][k] = sum_r[i][j][k - 1] + v,
                    None => sum_r[i][j][k] = sum_r[i][j][k - 1],
                }
            }
        }
    }
    //println!("{:?}", sum_d);
    //println!("{:?}", sum_r);
    
    // [i,j,k], [k,0]- [i,j]
    for j in 1..c+1{
        for i in 1..r+1 {
            for x in 1..i+1 {
                let mut t = 0u64;
                for _i in x..i+1 {
                    t += sum_d[_i][j][i];
                }
                cum_d[i][j][x] = cum_d[i][j-1][x] + t;
            }
        }
    }
    //println!("{:?}", cum_d);

    // [i,j,k]  [0,k]-[i,j]
    for i in 1..r+1 {
        for j in 1..c+1 {
            for y in 1..j+1 {
                let mut t = 0u64;
                for _j in y..j+1 {
                    t += sum_r[i][_j][j];
                }
                cum_r[i][j][y] = cum_r[i-1][j][y] + t;
            }
        }
    }
    //println!("{:?}", cum_r);

    let mut p: u64 = 0;
    let mut q: u64 = 1;
    let mut cnt: u64 = 0;

    for i in 1..r+1 {
        for j in 1..c+1 {
            for x in i..r+1 {
                for y in j..c+1 {
                    let DDD = cum_d[x][y][i] - cum_d[x][j-1][i];
                    let RRR = cum_r[x][y][j] - cum_r[i-1][y][j];
                    let LEN = DDD + RRR;
                    let w_h = (2+x+y-i-j) as u64;
                    //println!("{:?}", (i,j,x,y,LEN,w_h));
                    if LEN * q > p * w_h {
                        cnt = 1;
                        p = LEN; q = w_h;
                    } else if LEN * q == p * w_h {
                        cnt += 1;
                    }
                }
            }
        }
    }
    let g = gcd(p, q);
    println!("{}/{} {}", p/g, q/g, cnt);
}

pub fn main() {
    let T = read!(usize);
    for t in 1..T + 1 {
        print!("Case #{}: ", t);
        unsafe{solve();}
    }
}
