use std::cmp;
fn solve() -> i32 {
    let mut res = 111 * 333;
    for i in 100..=999 {
        for j in 100..=999 {
            if ok(i * j) {
                res = cmp::max(res, i * j);
            }
        }
    }
    res
}
pub fn ok(n: i32) -> bool {
    let s: Vec<char> = n.to_string().chars().collect();
    let n = s.len();
    for i in 0..n {
        if s[i] != s[n - 1 - i] {
            return false;
        }
    }
    true
}

pub fn ungly_ok(n: i32) -> bool {
    if n / 100000 == 0 {
        return n % 10 == n / 10000 && (n % 100) / 10 == (n / 1000) % 10;
    }
    if n / 1000000 == 0 {
        return n % 10 == n / 100000
            && (n % 100) / 10 == (n / 10000) % 10
            && (n % 1000) / 100 == (n / 1000) % 10;
    }
    n % 10 == n / 1000000
        && (n % 100) / 10 == (n / 100000) % 10
        && (n % 1000) / 100 == (n / 10000) % 10
}
pub fn main() {
    let res = solve();
    println!("{}", res);
}
