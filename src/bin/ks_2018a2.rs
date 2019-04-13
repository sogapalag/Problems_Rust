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

// Lucky Dip

use std::cmp;

fn compute(N: usize, K: usize, v: &Vec<f64>) -> f64 {
    let n = N as f64;
    let mut rsum = vec![0f64; N];
    (0..N).rev().fold(0f64, |acc, i| {
        let fac = acc + v[i];
        rsum[i] = fac;
        fac
    });
    let Ev = rsum[0] / n;
    // Ev[k+1] = t*E[k] + rsum[t]
    (1..K + 1).fold(Ev, |acc, _| {
        let idx = match v.binary_search_by(|probe| probe.partial_cmp(&acc).unwrap()) {
            Ok(id) => id,
            Err(id) => id,
        };
        (rsum[idx] + acc * idx as f64) / n
    })
}

fn solve(t: usize) {
    let mut nk: Vec<_>;
    read!(usize, nk);
    let N = nk[0];
    let K = nk[1];
    let mut v: Vec<_>;
    read!(f64, v);
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let res = compute(N, K, &v);

    println!("Case #{}: {:.6}", t, res);
}

pub fn main() {
    let T = read!(usize);
    for t in (1..T + 1) {
        solve(t);
    }
}
