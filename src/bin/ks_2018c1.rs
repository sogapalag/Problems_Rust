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
// Planet Distance

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

fn dfs(path: &mut vusz, tree: &Vec<vusz>, ok: &mut Vec<bool>, nxt: usize, last: usize) -> bool {
    //println!("{:?}", path);
    if ok[nxt] {
        return true;
    }
    ok[nxt] = true;
    for &x in &tree[nxt] {
        if x != last {
            path.push(x);
            if dfs(path, tree, ok, x, nxt) {
                return true;
            }
            path.pop();
        }
    }
    false
}

fn bfs(res: &mut vu32, tree: &Vec<vusz>, cycle: &[usize], ok: &mut Vec<bool>) {
    let mut nxro: vusz = Vec::with_capacity(res.len());
    let d = res[cycle[0]] + 1;
    for &c in cycle {
        for &nei in &tree[c] {
            if !ok[nei] {
                res[nei] = d;
                ok[nei] = true;
                nxro.push(nei);
            }
        }
    }
    if !nxro.is_empty() {
        bfs(res, tree, &nxro, ok);
    }
}

fn solve() {
    let n = read!(usize);
    let mut tree: Vec<vusz> = vec![vec![]; n + 1];
    for _ in 0..n {
        let mut v: Vec<_>;
        read!(usize, v);
        tree[v[0]].push(v[1]);
        tree[v[1]].push(v[0]);
    }
    //println!("{:?}", tree);

    // find cycle
    let mut path: vusz = Vec::with_capacity(n + 1);
    let mut ok = vec![false; n + 1];
    path.push(1);
    dfs(&mut path, &tree, &mut ok, 1, 0);

    let &co = path.last().unwrap();
    let mut i = 0;
    while path[i] != co {
        i += 1;
    }
    let cycle = &path[i + 1..];
    //println!("{:?}", cycle);
    let mut res = vec![0u32; n + 1];
    ok.iter_mut().for_each(|b| *b = false);
    cycle.iter().for_each(|&c| ok[c] = true);

    // comp dist.
    bfs(&mut res, &tree, &cycle, &mut ok);

    (1..n + 1).for_each(|i| print!(" {}", res[i]));
    println!("");
}
pub fn main() {
    let T = read!(usize);
    for t in 1..T + 1 {
        print!("Case #{}:", t);
        solve();
    }
}
