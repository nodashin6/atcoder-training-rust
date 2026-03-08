#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        pv: [(usize, usize); n],
    }

    // dp1[i][j] = 品物0..i-1を使い、重さちょうどjの最大価値 (累積maxなし)
    let mut dp1 = vec![vec![0; m + 1]; n + 1];
    for i in 0..n {
        let (p, v) = pv[i];
        dp1[i + 1] = dp1[i].clone();
        for j in 0..=(m - p) {
            dp1[i + 1][j + p] = dp1[i + 1][j + p].max(dp1[i][j] + v);
        }
    }
    let dp_max = *dp1[n].iter().max().unwrap();
    // dp2[i][j] = 品物i..n-1を使い、重さj以下の最大価値 (累積maxあり)
    let mut dp2 = vec![vec![0; m + 1]; n + 1];
    for i in (0..n).rev() {
        let (p, v) = pv[i];
        dp2[i] = dp2[i + 1].clone();
        for j in 0..=(m - p) {
            dp2[i][j + p] = dp2[i][j + p].max(dp2[i + 1][j] + v);
        }
        // 大は小を兼ねる
        for j in 0..m {
            dp2[i][j + 1] = dp2[i][j + 1].max(dp2[i][j]);
        }
    }
    let mut ans = vec!["A"; n];
    for i in 0..n {
        let (p, v) = pv[i];
        // item i を選んで最適解を達成できるか
        let mut can_select = false;
        for j in 0..=(m - p) {
            if dp1[i][j] + v + dp2[i + 1][m - j - p] == dp_max {
                can_select = true;
                break;
            }
        }
        // item i を選ばずに最適解を達成できるか
        let mut is_not_necessary = false;
        for j in 0..=m {
            if dp1[i][j] + dp2[i + 1][m - j] == dp_max {
                is_not_necessary = true;
                break;
            }
        }
        match (can_select, is_not_necessary) {
            (true, true) => ans[i] = "B",
            (true, false) => ans[i] = "A",
            (false, true) => ans[i] = "C",
            (false, false) => unreachable!(),
        }
    }
    println!("{}", ans.into_iter().join(""));
}
