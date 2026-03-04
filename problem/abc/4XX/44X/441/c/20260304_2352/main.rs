#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: u128,
        mut aa: [u128; n],
    }
    aa.sort();

    let mut ans = (n - k) as i64;
    let mut drank = 0;
    for i in (0..k).rev() {
        drank += aa[i];
        ans += 1;
        if drank >= x {
            break;
        }
    }
    if drank < x {
        ans = -1;
    }
    println!("{}", ans);
}
