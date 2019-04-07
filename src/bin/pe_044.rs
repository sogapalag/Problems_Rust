// DRAFT!!!,
//
// O(n^3), j could < i,.. this made me some time found the bug.
//
fn solve() {
    const LIM: usize = 5_000_000;
    let table: Vec<_> = (1..LIM).map(|x| (x * (3 * x - 1)) >> 1).collect();
    println!("{}", table[1911]);
    if let Ok(k) = table.binary_search(&92) {
        println!("{}", k);
    }
    for i in 1910..1915 {
        let pi = table[i];
        for j in 1..=(pi / 3) + 2 {
            let pj = table[j];
            let pk = pi + pj;
            if let Ok(k) = table.binary_search(&pk) {
                let pm = pk + pj;
                println!("1,k {} {} {}", i, k, pm);
                if let Ok(m) = table.binary_search(&pm) {
                    println!("i {}, pi {}", i, pi);
                    println!("j {}", j);
                    println!("k {:?}", k);
                    println!("m {:?}", m);
                }
            }
        }
    }
}

fn check(p: u64) -> bool {
    let cc = 1 + 24 * p;
    let c = (cc as f64).sqrt() as u64;
    if c * c != cc || c % 6 != 5 {
        return false;
    }
    true
}

fn P(n: u64) -> u64 {
    if n <= 0 {
        return 0;
    }
    (n * (3 * n - 1)) / 2
}

pub fn main() {
    //println!("{}", check(92));
    //println!("{}", check(145));
    //println!("{}", check(5482660));
    solve();
}
