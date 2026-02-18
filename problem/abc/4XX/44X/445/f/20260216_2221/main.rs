use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        k: usize,
        cc: [[usize; n]; n],
    }
    // g[i][j][d] = i番目からj番目まで距離2^{d}で移動するのに必要な最小コスト
    const INF: u64 = 1 << 60;
    const LOG_MAX: usize = 30;
    let mut g = vec![vec![vec![INF; LOG_MAX]; n]; n];
    for i in 0..n {
        for j in 0..n {
            g[i][j][0] = cc[i][j] as u64;
        }
    }
    for d in 1..LOG_MAX {
        for i in 0..n {
            for from in 0..n {
                for to in 0..n {
                    let new_dist = g[i][from][d - 1] + g[from][to][d - 1];
                    g[i][to][d] = min(g[i][to][d], new_dist);
                }
            }
        }
    }

    let mut ans = vec![0; n];
    for start in 0..n {
        let mut dp0 = vec![INF; n];
        let mut dp1 = vec![INF; n];
        dp0[start] = 0;
        for d in 0..LOG_MAX {
            if k & (1 << d) != 0 {
                for from in 0..n {
                    for to in 0..n {
                        let new_dist = dp0[from] + g[from][to][d];
                        dp1[to] = min(dp1[to], new_dist);
                    }
                }
                dp0 = dp1;
                dp1 = vec![INF; n];
            }
        }
        ans[start] = dp0[start];
    }

    for a in ans {
        println!("{}", a);
    }
}
