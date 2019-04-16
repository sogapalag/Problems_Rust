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

// AN UNGLY VERSION
// Sherlock and the Bit Strings
// TIME: ~ 19s, PASS the test.
// recurrsive version with ok check,
// with unsafe static array, and write_bytes(like memset), u64; (f64 may cause some accuracy error)

use std::cmp;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};
use std::{mem, ptr};

type vuu = Vec<[u32; 2]>;

static mut dp: &'static mut [[u64; (1u32 << 16) as usize]; 101] =
    &mut [[0u64; (1u32 << 16) as usize]; 101];
static mut ok: &'static mut [[bool; (1u32 << 16) as usize]; 101] =
    &mut [[false; (1u32 << 16) as usize]; 101];
//static mut cons: &'static mut [Vec<[u32;2]>; 101] = unsafe { mem::zeroed() };

static mut N: usize = 100;
unsafe fn rec(x: usize, b: usize, cons: &[vuu; 101]) -> u64 {
    //println!("{}", N);
    if ok[x][b] {
        dp[x][b]
    } else {
        ok[x][b] = true;
        if cons[x]
            .iter()
            .all(|&c| ((b >> c[0]) as usize).count_ones() == c[1])
        {
            if x == N {
                dp[x][b] = 1;
                1
            } else {
                dp[x][b] = rec(x + 1, b >> 1, cons).saturating_add(rec(x + 1, (b >> 1) + (1 << 15), cons));
                dp[x][b]
            }
        } else {
            dp[x][b] = 0;
            0
        }
    }
}

fn solve() {
    const K: usize = 15;
    const L: usize = 16;

    let mut v: Vec<_>;
    read!(u64, v);
    let (n, k, p) = (v[0] as usize, v[1] as usize, v[2]);
    unsafe {
        N = n;
    }
    let mut cons: [vuu; 101] = unsafe { mem::zeroed() };

    for _ in 0..k {
        let mut abc: Vec<_>;
        read!(u32, abc);
        //println!("{:?}", abc);
        unsafe {
            cons[abc[1] as usize].push([15 - (abc[1] - abc[0]), abc[2]]);
        }
    }
    //println!("{:?}", cons);

    //let dp_start = Instant::now();

    //let mut dp = vec![vec![0u64; 1 << L]; n + 1];

    //let dp_du = dp_start.elapsed();
    //println!("dp time{:?}", dp_du);

    //  for i in 1..n+1 {
    //      for j in 0..1<<16 {
    //          unsafe{ok[i][j] = false;}
    //      }
    //  }

    unsafe {
        ptr::write_bytes(ok.as_mut_ptr(), 0, 101);
    }

    unsafe {
        let mut p = p;
        let mut b = 0usize;
        for x in 1..n + 1 {
            if rec(x, b >> 1, &cons) >= p {
                print!("0");
                b >>= 1;
            } else {
                print!("1");
                p -= rec(x, b >> 1, &cons);
                b = (b >> 1) + (1 << K);
            }
        }
    }
    println!("");
}

pub fn main() {
    let T = read!(usize);
    //let total_start = Instant::now();
    for t in 1..T + 1 {
        print!("Case #{}: ", t);
        //let start = Instant::now();
        solve();
        //let duration = start.elapsed();
        //println!("Time elapsed in expensive_function() is: {:?}", duration);
    }
    //let total_du = total_start.elapsed();
    //println!("total time {:?}", total_du);
}
