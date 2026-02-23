use proconio::input;
use std::collections::VecDeque;

fn solve(n: usize) -> String {
    if n < 10 {
        return n.to_string();
    }
    // dp[mod_n][last_digit] = Some((prev_mod_n, prev_last_digit))
    // last_digit: 0=初期状態, 1-9=最後に追加した数字
    // dpに先に書き込まれたものが、より短く小さな数になることが保証される
    let mut dp: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; 10]; n];
    dp[0][0] = Some((0, 0)); // 初期状態（自分自身を指す）

    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    que.push_back((0, 0)); // (mod_n, last_digit)

    while let Some((prev_mod, prev_ld)) = que.pop_front() {
        let start = if prev_ld == 0 { 1 } else { prev_ld };
        for k in start..10 {
            let next_mod = (prev_mod * 10 + k) % n;
            if dp[next_mod][k].is_none() {
                dp[next_mod][k] = Some((prev_mod, prev_ld));
                que.push_back((next_mod, k));

                if next_mod == 0 {
                    return reconstruct(&dp, 0, k);
                }
            }
        }
    }

    "-1".to_string()
}

fn reconstruct(dp: &[Vec<Option<(usize, usize)>>], mod_n: usize, ld: usize) -> String {
    let mut digits = Vec::new();
    let mut cur_mod = mod_n;
    let mut cur_ld = ld;
    loop {
        if cur_ld == 0 {
            break;
        }
        digits.push(cur_ld as u8 + b'0');
        let (prev_mod, prev_ld) = dp[cur_mod][cur_ld].unwrap();
        cur_mod = prev_mod;
        cur_ld = prev_ld;
    }
    digits.reverse();
    String::from_utf8(digits).unwrap()
}

fn main() {
    input! {
        n: usize,
    }
    let ans = solve(n);
    println!("{}", ans);
}
