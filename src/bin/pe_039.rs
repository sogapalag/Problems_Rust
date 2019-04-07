fn solve() {
    let mut v = vec![0; 1001];
    for a in 1..500 {
        for b in a..500 {
            let cc = a * a + b * b;
            let c = (cc as f64).sqrt() as usize;
            if c * c == cc && a + b + c <= 1000 {
                v[a + b + c] += 1;
            }
        }
    }
    let res = v.iter().max();
    println!("{:?}", res);
    for i in 1..=1000 {
        if v[i] == 8 {
            println!("{}", i);
        }
    }
    // 840
}

pub fn main() {
    solve();
}
