use proconio::input;
use std::collections::HashMap;

type Factors = HashMap<usize, usize>;

const MOD: usize = 998244353;

// 逆元の列挙
fn calc_inv(n: usize) -> Vec<usize> {
    let mut inv = vec![0; n + 1];
    inv[1] = 1;
    for i in 2..=n {
        inv[i] = MOD - (MOD / i) * inv[MOD % i] % MOD;
    }
    inv
}

// 素因数分解
fn factors(mut n: usize) -> Factors {
    let mut factors = HashMap::new();
    let mut p = 2;
    while p * p <= n {
        while n % p == 0 {
            *factors.entry(p).or_insert(0) += 1;
            n /= p;
        }
        p += 1;
    }
    if n > 1 {
        *factors.entry(n).or_insert(0) += 1;
    }
    factors
}

fn solve(n: usize, a: Vec<usize>, invs: &[usize]) {
    let mut rank: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut b = vec![HashMap::new(); n];
    for i in 0..n {
        b[i] = factors(a[i]);
    }
    for i in 0..n {
        for (&p, &_e) in &b[i] {
            rank.entry(p).or_insert_with(Vec::new).push(i);
        }
    }
    let mut all_lcm = 1;
    let keys: Vec<usize> = rank.keys().cloned().collect();
    for &p in &keys {
        rank.get_mut(&p).unwrap().sort_by_key(|&i| b[i].get(&p).copied().unwrap_or(0));
        // pの最大指数を求める
        let indices = rank.get(&p).unwrap();
        let max_e = b[indices[indices.len() - 1]][&p];
        for _ in 0..max_e {
            all_lcm = (all_lcm * p) % MOD;
        }
    }

    // i番目だけを除外してlcmを求める
    // 指数が最大のとき, 2番目の指数との差分だけ逆元をかける
    let mut ans = vec![0; n];
    for i in 0..n {
        let factor = &b[i];
        let mut lcm = all_lcm;
        for (&p, &e) in factor {
            let indices = rank.get(&p).unwrap();
            let e_max = b[indices[indices.len() - 1]][&p];
            if e_max == e {
                let e_2nd = if indices.len() == 1 {
                    0
                } else {
                    b[indices[indices.len() - 2]][&p]
                };
                let diff = e - e_2nd;
                for _ in 0..diff {
                    lcm = (lcm * invs[p]) % MOD;
                }
            }
        }
        ans[i] = lcm;
    }
    print_with_space(&ans);
}

fn main() {
    const MAX_A: usize = 10000005;
    let invs = calc_inv(MAX_A);
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; n],
        }
        solve(n, a, &invs);
    }
}

fn print_with_space(a: &Vec<usize>) {
    println!(
        "{}",
        a.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
}
