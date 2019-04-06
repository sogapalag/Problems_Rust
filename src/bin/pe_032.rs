extern crate itertools;

use itertools::Itertools;

fn solve() {
    let mut v = vec![];
    // (2d) X (3d) = (4d)
    for i in 12..=98 {
        for j in 123..=987 {
            let si = digit_to_set(i);
            let sj = digit_to_set(j);
            if si & sj == 0 && (si | sj) & digit_to_set(i * j) == 0 {
                v.push(i * j);
                println!("{:?}", (i, j, i * j));
            }
        }
    }
    // (1d) X (4d) = (4d)
    for i in 1..=8 {
        for j in 1234..=9876 {
            let si = digit_to_set(i);
            let sj = digit_to_set(j);
            if si & sj == 0 && (si | sj) & digit_to_set(i * j) == 0 {
                v.push(i * j);
                println!("{:?}", (i, j, i * j));
            }
        }
    }
    let res: u32 = v.iter().unique().sum();
    println!("{}", res);
    //45228
}

fn digit_to_set(n: u32) -> u32 {
    let mut set: u32 = 0;
    let mut x = n;
    while x != 0 {
        let d = x % 10;
        // forbid zero
        if d == 0 {
            return 0b111_111_111;
        }
        let it = 1 << (d - 1);
        // distinct
        if it & set != 0 {
            return 0b111_111_111;
        }
        set |= it;
        x /= 10;
    }
    set
}

pub fn main() {
    solve();
}
