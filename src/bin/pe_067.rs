use std::cmp;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;

fn read() -> Vec<Vec<i32>> {
    let path = Path::new("src/bin/p067_triangle.txt");
    let f = File::open(&path).unwrap();
    let f = BufReader::new(f);
    let mut res: Vec<Vec<i32>> = Vec::new();
    for l in f.lines() {
        res.push(
            l.unwrap()
                .rsplit(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect(),
        );
    }
    res
}

fn solve() -> i32 {
    let mut v = read();
    let n = v.len();
    if n == 0 {
        return 0;
    }
    for i in (0..n - 1).rev() {
        for j in 0..=i {
            v[i][j] += cmp::max(v[i + 1][j], v[i + 1][j + 1]);
        }
    }
    //println!("{:?}", v);
    v[0][0]
}

pub fn main() {
    println!("{:?}", solve());
    //7273
}
