static FAC: &'static [u32] = &[1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

fn solve() {
    // create FAC[0..=9]
    // let mut fac = vec![1;10];
    // (1..=9).fold(1, |acc, i| {
    //     let x = acc*i;
    //     fac[i] = x;
    //     x
    // });
    // println!("{:?}", fac);
    // why L, since L ~ 9! * k < 99..9, k=7
    let L = 3_000_000;
    let mut sum = 0;
    for i in 3..L {
        if i == curious(i) {
            sum += i;
            println!("{}", i);
            // only 145, 40585
        }
    }
    println!("res, {}", sum);
}

fn curious(n: u32) -> u32 {
    let mut x = n;
    let mut res = 0;
    while x != 0 {
        let d = (x % 10) as usize;
        res += FAC[d];
        x /= 10;
    }
    res
}

pub fn main() {
    solve();
}
