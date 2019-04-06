extern crate primal;

fn solve(mut n: usize) -> usize {
    let mut v = vec![1; 10];
    let h = (1..10).fold(1, |acc, i| {
        let fac = acc * i;
        v[i] = fac;
        fac
    });
    let mut s = vec![0; 10];
    for i in 0..10 {
        s[i] = v[9 - i];
    }
    println!("{:?}", s);

    let mut t = vec![0; 10];
    for i in 0..10 {
        t[i] = i;
    }
    println!("{:?}", t);
    n = n - 1;
    let mut res = vec![100; 10];
    for i in 0..10 {
        let a = n / s[i];
        let b = n % s[i];
        res[i] = t.remove(a);
        n = b;
    }
    println!("{:?}res ", res);

    let mut ans = 0;
    let mut T = 1;
    for i in 0..10 {
        ans += res[9 - i] * T;
        T *= 10;
    }
    println!("{}", ans);
    ans
}
pub fn main() {
    solve(1000_000);
    //2783915460
}
