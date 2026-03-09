#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        tt: [i64; n],
    }
    let mut aa = tt
        .iter()
        .enumerate()
        .map(|(i, &t)| (t, i))
        .collect::<Vec<_>>();
    aa.sort();
    let ans = (0..3).into_iter().map(|i| aa[i].1 + 1).join(" ");
    println!("{}", ans);
}
