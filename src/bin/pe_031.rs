fn solve() {
    // dp[k][v]:= only in [0..=k]s, one can get value v,'s choices.
    let w = vec![1, 2, 5, 10, 20, 50, 100, 200];
    let mut dp = vec![vec![0; 201]; 8];
    for i in 0..8 {
        dp[i][0] = 1;
    }
    for i in 0..201 {
        dp[0][i] = 1;
    }

    for k in 1..8 {
        for v in 1..201 {
            let mut t = v;
            dp[k][v] = dp[k - 1][v];
            while t >= w[k] {
                dp[k][v] += dp[k - 1][t - w[k]];
                t -= w[k];
            }
        }
    }
    let res = dp[7][200];
    println!("{}", res);
    //73682
}

pub fn main() {
    solve();
}
