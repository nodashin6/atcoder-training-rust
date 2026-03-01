#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input;

// class
struct Solver {
    c: usize,
    a: Vec<(usize, usize)>,
    offset: usize,
}

impl Solver {
    fn new(c: usize, mut ww1: Vec<(usize, usize)>, ww2: Vec<(usize, usize)>, mut m: usize) -> Self {
        let mut a = Vec::new();
        let mut offset = 0;

        // ww1 is sorted descending. Reverse it to use pop() efficiently.

        while m > 0 && !ww1.is_empty() {
            let last = ww1.len() - 1;
            ww1[last].1 -= 1; // 1個減らす
            offset += ww1[last].0; // 価値を加算
            m -= 1;
            if ww1[last].1 == 0 {
                ww1.pop();
            }
        }
        while !ww1.is_empty() {
            let last = ww1.len() - 1;
            let count = ww1[last].1;
            let v = ww1[last].0;

            if count >= 2 {
                let pairs = count / 2;
                a.push((v * 2, pairs));
                ww1[last].1 %= 2;
            }

            if ww1[last].1 == 0 {
                ww1.pop();
                continue;
            }

            // count is 1
            ww1.pop();
            if !ww1.is_empty() {
                let last2 = ww1.len() - 1;
                let v2 = ww1[last2].0;
                ww1[last2].1 -= 1;
                if ww1[last2].1 == 0 {
                    ww1.pop();
                }
                a.push((v + v2, 1));
            } else {
                a.push((v, 1));
            }
        }
        for (v, k) in ww2 {
            a.push((v, k));
        }
        // v が小さい順にソート
        a.sort_by_key(|&(v, _)| v);
        // println!("a: {:?}, offest: {}", a, offset);
        Self { c, a, offset }
    }

    fn get_k(&self, k: usize) -> usize {
        // 価値の高い順に k 個選ぶ
        let mut total_value = 0;
        let mut total_k = 0;
        for &(v, count) in self.a.iter().rev() {
            let usage_k = count.min(k - total_k);
            total_value += v * usage_k;
            total_k += usage_k;
            if total_k == k {
                break;
            }
        }
        total_value
    }

    fn solve(&self, rem_c: usize) -> usize {
        let k = rem_c / 2; // すべてのアイテムの重さは2になっている.
        self.get_k(k) + self.offset // k 個のアイテムと, 最初のoffset分の価値を合計する
    }
}

fn get_k_w3(ww3: &[(usize, usize)], mut k: usize) -> usize {
    let mut total_value = 0;
    for &(v, count) in ww3.iter().rev() {
        let usage_k = count.min(k);
        total_value += v * usage_k;
        k -= usage_k;
        if k == 0 {
            break;
        }
    }
    total_value
}

fn solve_mid(solver_0: &Solver, solver_1: &Solver, ww3: &[(usize, usize)], mid: usize) -> usize {
    let value_w3 = get_k_w3(ww3, mid);
    let rem_c = solver_0.c - 3 * mid;
    let value_other = if rem_c > 0 {
        if rem_c & 1 == 0 {
            solver_0.solve(rem_c)
        } else {
            solver_1.solve(rem_c)
        }
    } else {
        0
    };
    // println!(
    //     "mid: {}, value_w3: {}, value_other: {}",
    //     mid, value_w3, value_other
    // );
    value_w3 + value_other
}

#[cfg(test)]
mod tests {}

fn main() {
    input! {
        n: usize,
        c: usize,
        wvk: [(usize, usize, usize); n],
    }
    const BRUTE_FORCE_LIMIT: usize = 100;
    let mut www = vec![Vec::new(); 3];
    for (w, v, k) in wvk {
        www[w - 1].push((v, k));
    }
    // v が小さい順にソート
    for i in 0..3 {
        www[i].sort_by_key(|&(v, _)| v);
    }

    // w=3の入れる数量を三分探索する
    // その後の w=1 の商品を 2つセットで計算することで, 貪欲に求めることができる
    // ただし空き容量が奇数のとき, 最良の w=1 の商品を選ぶことは確定していることに注意
    let solver_0 = Solver::new(c, www[0].clone(), www[1].clone(), 0);
    let solver_1 = Solver::new(c, www[0].clone(), www[1].clone(), 1);
    let mut lo = 0;
    let mut hi = c / 3 + 1;
    while hi - lo > BRUTE_FORCE_LIMIT {
        let mid1 = (2 * lo + hi) / 3;
        let mid2 = (lo + 2 * hi) / 3;
        let score1 = solve_mid(&solver_0, &solver_1, &www[2], mid1);
        let score2 = solve_mid(&solver_0, &solver_1, &www[2], mid2);
        if score1 < score2 {
            lo = mid1;
        } else {
            hi = mid2;
        }
    }
    let mut ans = 0;
    for mid in lo..hi {
        ans = ans.max(solve_mid(&solver_0, &solver_1, &www[2], mid));
    }
    println!("{}", ans);
}
