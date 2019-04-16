// fenwick, aka. (BIT) binary indexed tree
// this is one_based impl
// check these out:
// https://www.topcoder.com/community/competitive-programming/tutorials/binary-indexed-trees/
// https://en.wikipedia.org/wiki/Fenwick_tree
// the key idea is each idx responsible [idx - LSB(i) + 1, idx]
// and key trick is compute LSB or idx +- LSB faster
// cpp trick LSB(i) = i & -i

use std::ops::{AddAssign, Sub};

// update aka. tree[idx], [+LSB]...
pub fn update<T>(fenwick: &mut [T], mut idx: usize, val: T)
where
    T: AddAssign + Copy + Default,
{
    while idx < fenwick.len() {
        fenwick[idx] += val;
        idx = (idx | (idx - 1)) + 1;
    }
}
// sum in [1, idx], aka. tree[idx], [-LSB]...
pub fn query<T>(fenwick: &[T], mut idx: usize) -> T
where
    T: AddAssign + Copy + Default,
{
    let mut sum = T::default();
    while idx > 0 {
        sum += fenwick[idx];
        idx &= idx - 1;
    }
    sum
}

pub fn qr<T>(fenwick: &[T], left: usize, right: usize) -> T
where
    T: AddAssign + Copy + Default + Sub<Output = T>,
{
    query(fenwick, right) - query(fenwick, left - 1)
}
pub fn main() {
    let array = [0; 100];
    qr(&array, 1, 3);
}
