#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut uu = vec![(n - 1) as i64; n];
    for (a, b) in ab {
        uu[a - 1] -= 1;
        uu[b - 1] -= 1;
    }
    let mut ans = vec![0i64; n];
    for (i, u) in uu.into_iter().enumerate() {
        ans[i] = u * (u - 1) * (u - 2) / 6;
    }

    println!("{}", ans.iter().join(" "))
}
