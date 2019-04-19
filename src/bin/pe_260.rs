// it's consider empty opration.
// but view as this would be easy.
// when some LOSS claim the empty way. 
// The behinder no longer claim.
// e.g. 035 claim 002 although 002 itself is not a LOSS, 
// 035 claim all the possible empty into '002', 
// thus mirrorly 046 cannot claim 002 again.

fn solve() {
    const n: usize = 1000;
    const N: usize = n+5;
    let mut crown = vec![vec![vec![false;N];N];3];
    let mut res = 0;
    for x in 0..=n{
        for y in x..=n {
            for z in y..=n {
                if crown[0][y][z] || crown[0][x][z] || crown[0][x][y] {
                    continue;
                }
                if crown[1][y-x][z] || crown[1][z-y][x] || crown[1][z-x][y] {
                    continue;
                }
                if crown[2][y-x][z-x] {
                    continue;
                }
                crown[0][y][z] = true;
                crown[0][x][z] = true;
                crown[0][x][y] = true;
                crown[1][y-x][z] = true;
                crown[1][z-y][x] = true;
                crown[1][z-x][y] = true;
                crown[2][y-x][z-x] = true;
                res += x + y + z;
            }
        }
    }
    println!("{}", res);
    //167542057
}

pub fn main() {
    solve();
}
