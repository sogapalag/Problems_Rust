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
// Scrambled Words
//
// with_capacity, fast
// be careful with collect, alloc request;

use std::cmp;
use std::collections::{HashMap, HashSet};

fn solve(t: usize) {
    let L = read!();
    let mut v: Vec<_>;
    read!(String, v);
    let mut ssnabcd: Vec<_>;
    read!(String, ssnabcd);

    // CREATE DICT
    let mut dict = HashMap::new();
    let mut len_set = HashSet::new();
    v.iter().for_each(|w| {
        let wlen = w.chars().count();
        len_set.insert(wlen);
        let mut freq = [0u32; 26];
        w.chars().for_each(|c| {
            let idx = (c as u8 - 97) as usize;
            freq[idx] += 1;
        });
        let SE = [w.as_bytes()[0], w.as_bytes()[wlen - 1]];

        let counter = dict.entry((SE, freq)).or_insert(0);
        *counter += 1;
    });
    //println!("{:?}", dict);
    //println!("{:?}", len_set);

    let s1 = ssnabcd[0].as_bytes()[0];
    let s2 = ssnabcd[1].as_bytes()[0];
    let n = ssnabcd[2].parse::<usize>().unwrap();
    let abcd: Vec<_> = ssnabcd[3..]
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let a = abcd[0];
    let b = abcd[1];
    let c = abcd[2];
    let d = abcd[3];

    // CREATE STRING
    let mut longs: Vec<u8> = Vec::with_capacity(n);
    longs.push(s1);
    longs.push(s2);
    let (mut x1, mut x2) = (s1 as u64, s2 as u64);
    (2..n).for_each(|_| {
        let x3 = (a * x2 + b * x1 + c) % d;
        x1 = x2;
        x2 = x3;
        let ret = 97 + (x3 % 26) as u8;
        longs.push(ret);
    });

    // COUNT FREQ
    let mut freq = [0u32; 26];
    let mut longs_counter: Vec<[u32; 26]> = Vec::with_capacity(n);
    longs.iter().for_each(|c| {
        freq[(c - 97) as usize] += 1;
        longs_counter.push(freq);
    });

    // loop length, potential words.
    let mut res = 0;
    len_set.iter().for_each(|l| {
        (0..n - l + 1).for_each(|w_s| {
            let w_e = w_s + l - 1;
            let SE = [longs[w_s], longs[w_e]];
            (0..26).for_each(|i| {
                freq[i] = longs_counter[w_e][i] - longs_counter[w_s][i];
            });
            let base = (longs[w_s] - 97) as usize;
            freq[base] += 1;
            if let Some(val) = dict.remove(&(SE, freq)) {
                res += val;
            }
        });
    });
    //println!("{:?}", longs);
    //
    println!("Case #{}: {}", t, res);
}

pub fn main() {
    let T = read!(usize);
    for t in (1..T + 1) {
        solve(t);
    }
}
