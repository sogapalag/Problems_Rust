extern crate primal;

fn solve() {
    let leading: &[u64] = &[23, 29, 31, 37, 53, 59, 71, 73, 79];
    let ending: &[u64] = &[13, 23, 43, 53, 73, 83, 17, 37, 47, 67, 87, 97];
    for l in leading {
        for e in ending {
            let p = l * 100 + e;
            if check(p) {
                println!("{}", p);
                //3137, 3797
            }
            for i in 1..=99 {
                let _p = l * 10000 + (i as u64) * 100 + e;
                if check(_p) {
                    println!("{}", _p);
                }
            }
        }
    }
    for l in leading {
        for i in &[1u64, 3, 5, 7, 9] {
            let p = l * 10 + i;
            if check(p) {
                println!("{}", p);
            }
        }
    }
    // one easy find
    // 23, 53, 73, 37
    // 313, 373, 317, 797
    // combine 3137, 3797
    // luckily find 739397
    let v: &[u64] = &[23, 53, 73, 37, 313, 373, 317, 797, 3137, 3797, 739397];
    for p in v {
        println!("{}", check(*p));
    }
    let sum = v.iter().fold(0, |acc, i| acc + i);
    println!("sum {}", sum);
}

fn check(p: u64) -> bool {
    let mut x = p;
    let mut b = 1;
    while x != 0 {
        if !primal::is_prime(x) {
            return false;
        }
        x /= 10;
        b *= 10;
    }
    x = p;
    while x != 0 {
        if !primal::is_prime(x) {
            return false;
        }
        x %= b;
        b /= 10;
    }
    true
}
pub fn main() {
    solve();
}
