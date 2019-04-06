extern crate chrono;

use chrono::prelude::*;

fn sundays_in_Jans(start_yr: i32, end_yr: i32) -> u32 {
    let mut res: u32 = 0;
    for yr in start_yr..=end_yr {
        // 0~6
        let n = Utc.ymd(yr, 1, 1).weekday().num_days_from_monday();
        // 4,5,6 Fri.~
        res += (n + 31) / 7;
    }
    res
}

// in each month, sunday is first
fn sundays_first(s: i32, e: i32) -> u32 {
    let mut res: u32 = 0;
    for yr in s..=e {
        for mm in 1..=12 {
            res += Utc.ymd(yr, mm, 1).weekday().number_from_monday() / 7;
        }
    }
    res
}

pub fn main() {
    let res = sundays_first(1901, 2000);
    println!("{}", res);
    //let start = Utc.ymd(1901, 1, 1);
    //let end = Utc.ymd(2000, 12, 31);
    //let dua = end - start;
    //let days = dua.num_days();
    //let k = days % 7;
    //let res = days / 7;
    //println!("{:?}", (res, k));
}
