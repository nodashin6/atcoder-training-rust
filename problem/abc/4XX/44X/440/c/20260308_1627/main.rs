#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input;

fn solve() {
    input! {
        n: usize,
        w: usize,
        cc: [i64; n],
    }
    let mut aa = vec![0; 4 * w + 1];
    for i in 0..n {
        let j = i % (2 * w);
        aa[j + 1] += cc[i];
        aa[j + 2 * w + 1] += cc[i];
    }
    for j in 1..(4 * w + 1) {
        aa[j] += aa[j - 1];
    }
    let mut ans = i64::MAX;
    for x in 0..2 * w {
        let score = (aa[x + w] - aa[x]) as i64;
        ans = ans.min(score);
    }
    println!("{}", ans);
}

fn main() {
    input! {
        t: i64,
    }
    for _ in 0..t {
        solve();
    }
}
