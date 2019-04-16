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

// READ ./notes/ks_2018b2.md may helpful.

// Sherlock and the Bit Strings
// dp loop version, without unsafe, use u64; total time:= 100*1.9s ~ 200s

use std::cmp;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

fn solve() {
    const K: usize = 15;
    const L: usize = 16;

    let mut v: Vec<_>;
    read!(u64, v);
    let (n, k, p) = (v[0] as usize, v[1] as usize, v[2]);
    let mut cons = vec![vec![]; 101];
    for _ in 0..k {
        let mut abc: Vec<_>;
        read!(usize, abc);
        //println!("{:?}", abc);
        cons[abc[1]].push((K - (abc[1] - abc[0]), abc[2] as u32));
    }
    //println!("{:?}", cons);

    let dp_start = Instant::now();

    let mut dp = vec![vec![0u64; 1 << L]; n + 1];
    for x in (1..n + 1).rev() {
        let co = cons[x].clone();
        for b in 0..1 << L {
            if co.iter().all(|&c| ((b >> c.0) as usize).count_ones() == c.1) {
                if x == n {
                    dp[x][b] = 1;
                } else {
                    // MAX instead overflow
                    dp[x][b] = dp[x + 1][b >> 1].saturating_add(dp[x + 1][(b >> 1) + (1 << K)]);
                }
            }
        }
    }

    let dp_du = dp_start.elapsed();
    println!("dp time{:?}", dp_du);

    let mut p = p;
    let mut b = 0usize;
    for x in 1..n + 1 {
        if dp[x][b>>1] >= p {
            print!("0");
            b >>= 1;
        } else {
            print!("1");
            p -= dp[x][b>>1];
            b = (b >> 1) + (1 << K);
        }
    }
    println!("");
}

pub fn main() {
    let T = read!(usize);
    let total_start = Instant::now();
    for t in 1..T + 1 {
        print!("Case #{}: ", t);
        let start = Instant::now();
        solve();
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }
    let total_du = total_start.elapsed();
    println!("total time {:?}", total_du);
}
