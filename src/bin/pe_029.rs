// for
// 5->25, 6->36, 7->49, 10->100, b~(2,100)
// each has same 49s.
// for
// 3->9->27->81, (1,2,3,4)X(2-100)
// for
// 2->4->8->16->32->64, (1,2,3,4,5,6)X(2-100)

use std::collections::HashSet;

fn solve() {
    let mut set = HashSet::new();
    for i in 1..=6 {
        for j in 2..=100 {
            set.insert(i * j);
        }
        if i % 2 == 0 {
            println!("{}", i * 99 - set.len());
            // 49, 156, 266;
        }
    }
    let rep = 49 * 4 + 156 + 266;
    let ans = 99 * 99 - rep;
    println!("{}", ans);
    //9183
}

pub fn main() {
    solve();
}
