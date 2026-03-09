#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        x: i64,
        y: u32,
    }
    let k = 2_i64.pow(y);
    println!("{}", x * k);
}
