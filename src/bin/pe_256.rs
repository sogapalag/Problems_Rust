// https://oeis.org/A068920/a068920.txt
// lemma, left-most 1x2 cannot appear except top or bottom.
// => when r odd, only rX(r-1), rX(r+1). valid, thus, r-1, r+1 compose s.
//    when r even, only rX(r-2), rXr, rX1 valid, thus
//    def 1 with pattern, a1a; r-2, with b(r-2)b;
//    r with a-a b|b, note a(r)a can be viewed 1,r-2,1. so wlog consider b(r)b
//    THEN, r,r-2 is "0", 1 is "1", valid 0101010
//    a 01 is 1 step with r-1 or r+1, so compose any s-1, s, s+1 is valid.

fn compo(r: usize, s: usize) -> bool {
    // check s =? a * (r-1) + b * (r + 1), with a,b >= 0 solution.
    let n = s / r;
    let dl = s - n * r;
    let dr = r - dl;
    (n >= dl && (n - dl) & 1 == 0) || (n + 1 >= dr && (n + 1 - dr) & 1 == 0)
}

// check tatami-free
fn tatami(r: usize, s: usize) -> bool {
    if r > s {
        return tatami(s, r);
    }
    if r & 1 != 0 {
        return compo(r, s);
    }
    compo(r, s - 1) || compo(r, s) || compo(r, s + 1)
}

// O(N^{3/2}) rustc -O with ~30min
fn naive() {
    const N: usize = 100_000_000;
    const S: usize = 300;
    let target = 200;
    let mut T = vec![0; S];
    for i in (2..N).filter(|&x| x & 1 == 0) {
        let mut cnt = 0;
        let sqi = (i as f64).sqrt() as usize;
        for div in 1..=sqi {
            if i % div == 0 && !tatami(div, i / div) {
                cnt += 1;
            }
        }
        if cnt != 0 && T[cnt] == 0 {
            T[cnt] = i;
            println!("i{}, cnt{}", i, cnt);
        }
        if cnt == target {
            break;
        }
    }
    println!("res {}", T[target]);
    //85765680
}

use std::cmp;

// 1 second O(N)
fn dp() {
    const N: usize = 100_000_000;
    const target: usize = 200;
    let mut T = vec![0; 5 + N >> 1];
    let SQN = (N as f64).sqrt() as usize;
    for i in 5..=SQN {
        let up = cmp::min(N / i, i * i);
        for j in i..=up {
            if (i * j) & 1 == 0 && !tatami(i, j) {
                T[(i * j) >> 1] += 1;
            }
        }
    }
    //println!("{:?}", T[35]);
    if let Some(res) = T.iter().position(|&x| x == 200) {
        println!("{}", res << 1);
    }
}

pub fn main() {
    dp();
}
