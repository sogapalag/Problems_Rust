pub fn main() {
    let mut res = 0;
    let mut n = 0;
    for i in 1..=100 {
        res += i * i;
        n += i;
    }
    res = n * n - res;
    println!("{}", res);
}
// or use formula.
