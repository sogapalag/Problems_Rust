

fn collatz(mut n: usize) -> usize {
    // assume >0
    let mut step = 0;
    while n != 1{
        if n & 1 == 0 {
            n >>= 1;
        }
        else{
            n = 3*n + 1;
        }
        step += 1;
    }
    step
}

fn find_largest(LIMIT: usize) -> (usize, usize){
    let mut res = (1, 0);
    for i in 1..LIMIT{
        let step = collatz(i);
        if step > res.1 {
            res = (i, step);
        }
    }
    res
}

pub fn main() {
    const LIMIT:usize = 1000_000;
    let res = find_largest(LIMIT);
    println!("{:?}", res);
}
