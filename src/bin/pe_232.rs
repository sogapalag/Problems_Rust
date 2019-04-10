// one can prove P = max_x f(x)P + g(x) <=> P = max_x g(x)/(1-f(x))
// thus we derive 1,2,4,8..step's maximum, recurrence.(bayes)
// 
// note: f64 doesn't have Ord trait, cause NaN?
// so can't max, but we use fold(0.0, f64::max)

fn solve() {
    // let mut q: Vec<f64> = vec![1f64; 10];
    // let mut s: Vec<usize> = vec![0; 10];
    // (1..10).fold(1f64, |acc, i| {
    //     let fac = acc/2.0;
    //     q[i] = fac;
    //     fac
    // });
    // (1..10).fold(1, |acc, i|{
    //     s[i] = acc;
    //     acc * 2
    // });
    // println!("{:?}", q);
    // println!("{:?}", s);
    let q: &[f64] = &[
        1.0,
        0.5,
        0.25,
        0.125,
        0.0625,
        0.03125,
        0.015625,
        0.0078125,
        0.00390625,
        0.001953125,
    ];
    let s: &[usize] = &[0, 1, 2, 4, 8, 16, 32, 64, 128, 256];
    // Prob when player b's action, remain scores for b,a: i, j
    let mut Pb = vec![vec![0f64; 101]; 101];
    // note i=0 still 1, since we consider Pb's starting view.
    for i in 0..=100 {
        Pb[i][0] = 1.0;
    }
    for i in 1..=100 {
        for j in 1..=100 {
            let pb: f64 = (1..9)
                .map(|t| {
                    let qt = q[t];
                    let _j = if j > s[t] { j - s[t] } else { 0 };
                    let pbt =
                        (Pb[i - 1][j] * (1.0 - qt) + (Pb[i - 1][_j] + Pb[i][_j]) * qt) / (1.0 + qt);
                    pbt
                })
                .fold(0.0, f64::max);
            Pb[i][j] = pb;
        }
    }
    let Pa_start = 0.5 * ((1.0 - Pb[99][100]) + (1.0 - Pb[100][100]));
    let res = 1.0 - Pa_start;
    println!("{:?}", res);
    //0.8364855558469472
    println!("{:.8}", res);
    //0.83648556
}

pub fn main() {
    solve();
}
