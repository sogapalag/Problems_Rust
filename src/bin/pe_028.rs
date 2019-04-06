// watch right-up, 1, 3^2, 5^2, 7^2,...
// note every dist _, 2,  4  ,  6, ....

fn solve(n: usize) {
    let mut res: usize = 1;
    for k in 1..=n {
        res += 4 * (2 * k + 1) * (2 * k + 1) - 12 * k;
    }
    println!("{}", res);
}

pub fn main() {
    solve(500);
    //669171001
}
