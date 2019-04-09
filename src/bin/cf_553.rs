//#[macro_use]
//extern crate text_io;
// the text_io may bugs with '\n','\r'
// when run locally passed, but OJ fail;
// So I write the ugly unwrap...
// maybe I should write my macro, or use unsafe?.

use std::io::{self, BufRead};

fn solve(n: usize, m: usize, k: u32, w: &Vec<u32>, c: &Vec<u32>) {
    // accum.
    let mut tw = Vec::with_capacity(n + 1);
    let mut tc = Vec::with_capacity(n + 1);
    w.iter().fold(0, |acc, i| {
        let fac = acc + i;
        tw.push(fac);
        fac
    });
    c.iter().fold(0, |acc, i| {
        let fac = acc + i;
        tc.push(fac);
        fac
    });
    // HH...HT...T must be one of optimal.
    let res = (m + 1..=n)
        .rev()
        .take_while(|&i| tw[i] - tw[i - m] <= k * tw[i - m])
        .map(|i| {
            let target = tw[i - m] - (tw[i] - tw[i - m] + k - 1) / k;
            let take_t = match tw.binary_search(&target) {
                Ok(idx) => idx,
                Err(idx) => idx - 1,
            };
            let val = tc[n] - tc[i] + tc[take_t];
            (n - i, take_t, val)
        })
        .max_by_key(|tup| tup.2);
    let res = match res {
        Some(r) => r,
        _ => (0, 0, 0),
    };
    println!("{} {}", res.0 + res.1, res.2);
    if res.0 + res.1 != 0 {
        println!("{}{}", "H".repeat(res.0), "T".repeat(res.1));
    }
}

pub fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer);
    let mut iter = buffer.split(' ').map(|s| s.trim().parse::<usize>());

    let n: usize;
    let m: usize;
    let k: u32;

    n = iter.next().unwrap().unwrap();
    m = iter.next().unwrap().unwrap();
    k = iter.next().unwrap().unwrap() as u32;
    //println!("{:?}x", (n,m,k));
    let mut w = Vec::with_capacity(n + 1);
    let mut c = Vec::with_capacity(n + 1);
    w.push(0);
    c.push(0);
    for _ in 0..n {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer);
        let mut iter = buffer.split(' ').map(|s| s.trim().parse::<u32>());
        w.push(iter.next().unwrap().unwrap());
        c.push(iter.next().unwrap().unwrap());
    }
    //println!("{:?}", w);
    //println!("{:?}", c);
    solve(n, m, k, &w, &c);
}
