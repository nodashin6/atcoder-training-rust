#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        sss: [String; n],
    }

    // dp[x][y] = x行目のy文字目までを `.`, y+1 文字目以降を `#` に変更するコストの最小値
    // min(dp[x-1][y:]) + sss[x][y:].count("#") + sss[x][:y].count(".")
    // すべて累積和や累積最小値をあらかじめ求めておけば、O(1) で取得できる
    let mut dp = vec![vec![0i64; n + 1]; n + 1];

    for x in 1..=n {
        let s = sss[x - 1].as_bytes();

        // hash_prefix[y] = sss[x-1][0..y] 中の '#' の数
        let mut hash_prefix = vec![0i64; n + 1];
        for i in 0..n {
            hash_prefix[i + 1] = hash_prefix[i] + if s[i] == b'#' { 1 } else { 0 };
        }

        // dot_suffix[y] = sss[x-1][y..n] 中の '.' の数
        let mut dot_suffix = vec![0i64; n + 1];
        for i in (0..n).rev() {
            dot_suffix[i] = dot_suffix[i + 1] + if s[i] == b'.' { 1 } else { 0 };
        }

        // suffix_min[y] = min(dp[x-1][y..=n])
        let mut suffix_min = vec![0i64; n + 2];
        suffix_min[n + 1] = i64::MAX;
        for j in (0..=n).rev() {
            suffix_min[j] = suffix_min[j + 1].min(dp[x - 1][j]);
        }

        for y in 0..=n {
            dp[x][y] = suffix_min[y] + hash_prefix[y] + dot_suffix[y];
        }
    }

    let ans = dp[n].iter().min().unwrap();
    println!("{}", ans);
}
