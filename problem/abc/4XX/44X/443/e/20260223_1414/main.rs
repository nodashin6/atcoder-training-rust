use proconio::input;
use proconio::marker::Bytes;

fn solve() {
    input! {
        n: usize,
        c: usize,
        sss: [Bytes; n],
    }
    let w = n; // 列幅
    let c = c - 1;
    let mut dp0 = vec![0; w];
    dp0[c] = 1; // start from c
    let mut wall = vec![0; w];
    for (i, &s) in sss[n-1].iter().enumerate() {
        if s == b'#' {
            wall[i] = 1;
        }
    }

    for i in (1..n).rev() {
        let mut dp1 = vec![0; w];
        let j = i - 1;
        for center in 0..w {
            if dp0[center] == 0 {
                continue;
            }
            for dk in [!0, 0, 1] {
                let k = center.wrapping_add(dk);
                if k >= w {
                    continue;
                }
                if sss[j][k] == b'.' {
                    dp1[k] = 1;
                } else if wall[k] == 0 {
                    // 壁があるけど、壊すことができる
                    dp1[k] = 1;
                } else {
                    // 壁があるし、壊すこともできない
                    dp1[k] = 0;
                }
            }
        }
        for center in 0..w {
            // 移動できていない かつ　壁がある --> wall[center] = 1
            if dp1[center] == 0 && sss[j][center] == b'#' {
                wall[center] = 1;
            }
        }
        dp0 = dp1;
    }

    let ans: String = dp0.iter().map(|v| v.to_string()).collect::<Vec<_>>().join("");
    println!("{}", ans);
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        solve();
    }
}
