extern crate ndarray;

use ndarray::{arr1, arr2, Array1};

// Consider right situations
// RRDLDRDLL  f[n-1]
// RRDDDLL subcases(df[n]): point (2,n-1) (3,n-1) must passed in some way
//           one before, one after,i.e. symmetry, df[n-2] = f[n-2]-f[n-3]
//           two before(after)(X2): f[n-3]+f[n-4]+...+f[1]
fn naive() {
    let mut f = vec![0, 1, 1, 4];
    let mut s = vec![0, 1, 2, 6];
    (4..11).fold(6, |acc, i| {
        let fi = s[i - 1] + s[i - 2] + f[i - 2] - f[i - 3];
        let fac = acc + fi;
        f.push(fi);
        s.push(fac);
        fac
    });
    println!("{}", f[10]);
    // 2329
}

// MGF does't give us simple answer
// let's try matrix

// S_n = 2S_{n-1} + 2S_{n-2} - 2S_{n-3} + S_{n-4}
fn solve() {
    let N: u64 = 1_000_000_000_000; //F_N
    let m: u64 = 100_000_000; //mod
                              // transition matrix
    let A = arr2(&[
        [2u64, 2, m - 2, 1],
        [1, 0, 0, 0],
        [0, 1, 0, 0],
        [0, 0, 1, 0],
    ]);
    let k: usize = 48; // >12*log2(10)
                       // a[k] = A^{2^k}
    let mut a = vec![A.clone()];
    (1..=k).fold(A, |acc, _| {
        let fac = acc.dot(&acc).map(|x| x % m);
        a.push(fac.clone());
        fac
    });

    let I = arr2(&[[1u64, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]);
    let aN = (0..=k).fold(I, |acc, i| {
        if N & (1 << i) != 0 {
            acc.dot(&a[i]).map(|x| x % m)
        } else {
            acc
        }
    });
    println!("{:?}", aN);
    let mut res = m + aN[[1, 0]] - aN[[2, 0]];
    res %= m;
    println!("{}", res);
    // 15836928
}

pub fn main() {
    //naive();
    solve();
}
