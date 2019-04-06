fn solve() {
    let mut p = 1;
    let mut q = 1;
    for i in 10..=99 {
        for j in i + 1..=99 {
            if i % 10 == j / 10 && i * (j % 10) == j * (i / 10) {
                // use the property
                p *= i / 10;
                q *= j % 10;
                println!("{:?}", (i, j));
            }
        }
    }
    println!("{:?}", (p, q));
    // 8, 800, thus answer 100.
}

pub fn main() {
    solve();
}
