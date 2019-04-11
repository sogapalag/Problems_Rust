extern crate num_integer;
extern crate num_traits;

use num_bigint::BigUint;
use num_integer::binomial;
use num_traits::pow;

// fix pow overflow bug
// without BigUint
fn compute(n: usize, p: usize, t: usize, s: usize) -> u128 {
    // n: #toss, 20
    // p: #sides, 12
    // t: #tops, 10
    // s: sum,  70
    use std::cmp::min;
    let mut bino = vec![vec![0u128; n + 1]; n + 1];
    for i in 0..=n {
        for j in 0..=i {
            let bij = binomial(i as u128, j as u128);
            bino[i][j] = bij;
        }
    }
    let dp: &mut Vec<_> = &mut vec![vec![vec![vec![0u128; s + 1]; t + 1]; p + 1]; n + 1];
    for i in 0..=n {
        for sides in 0..=p {
            dp[i][sides][0][0] = pow(sides as u128, i);
        }
    }
    for i in 1..=n {
        for sides in 1..=p {
            for top in 1..=min(t, i) {
                for score in top..=s {
                    // exactly k roll = sides
                    for k in 0..=min(top, score / sides) {
                        dp[i][sides][top][score] +=
                            bino[i][k] * dp[i - k][sides - 1][top - k][score - k * sides];
                    }
                    // when just reach, can have many same the sides
                    // this is important, otherwise bug
                    if score % sides == 0 && score / sides == top {
                        for k in top + 1..=i {
                            dp[i][sides][top][score] += bino[i][k] * dp[i - k][sides - 1][0][0];
                        }
                    }
                }
            }
        }
    }
    dp[n][p][t][s]
}

fn magic(n: usize, p: usize, t: usize, s: usize) -> BigUint {
    use std::cmp::min;

    let mut bino = vec![vec![BigUint::from(0u8); n + 1]; n + 1];
    for i in 0..=n {
        for j in 0..=i {
            let bij = BigUint::from(binomial(i, j));
            bino[i][j] = bij;
        }
    }
    let dp: &mut Vec<_> =
        &mut vec![vec![vec![vec![BigUint::from(0u8); s + 1]; t + 1]; p + 1]; n + 1];
    for i in 0..=n {
        for sides in 0..=p {
            // ...pow(n,k), k must be usize....omg; and n usize will overflow
            // and overflow firs with sides as usize.. damn.
            // now the code is ugly with BigUint, which dp need clone..
            dp[i][sides][0][0] = BigUint::from(pow::<u128>(sides as u128, i));
        }
    }
    for i in 1..=n {
        for sides in 1..=p {
            for top in 1..=min(t, i) {
                for score in top..=s {
                    // exactly k roll = sides
                    for k in 0..=min(top, score / sides) {
                        dp[i][sides][top][score] = dp[i][sides][top][score].clone()
                            + bino[i][k].clone()
                                * dp[i - k][sides - 1][top - k][score - k * sides].clone();
                    }
                    // when just reach, can have many same the sides
                    // this is important, otherwise bug
                    if score % sides == 0 && score / sides == top {
                        for k in top + 1..=i {
                            dp[i][sides][top][score] = dp[i][sides][top][score].clone()
                                + bino[i][k].clone() * dp[i - k][sides - 1][0][0].clone();
                        }
                    }
                }
            }
        }
    }
    dp[n][p][t][s].clone()
}

fn solve() {
    //let res = compute(5, 6, 3, 15);
    //println!("{}", res);
    // 111
    let res = compute(20, 12, 10, 70);
    println!("{}", res);
    // same as below
    let ans = magic(20, 12, 10, 70);
    println!("{}", ans);
    //7448717393364181966
}

pub fn main() {
    solve();
}
