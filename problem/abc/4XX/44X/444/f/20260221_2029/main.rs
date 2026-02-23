use proconio::input;

struct MapEntry {
    lo_key: usize,
    lo_count: i64,
    hi_key: usize,
    hi_count: i64,
    divide_cnt: i64,
}

fn divide(entry: &MapEntry) -> MapEntry {
    let new_lo_key = entry.lo_key / 2;
    let new_hi_key = entry.lo_key / 2 + 1;
    let mut new_lo_count = 0;
    let mut new_hi_count = 0;
    let new_divide_cnt = entry.divide_cnt + entry.lo_count + entry.hi_count;
    if entry.lo_key % 2 == 0 {
        new_lo_count += entry.lo_count * 2 + entry.hi_count;
        new_hi_count += entry.hi_count;
    } else {
        new_lo_count += entry.lo_count;
        new_hi_count += entry.lo_count + entry.hi_count * 2;
    };
    MapEntry {
        lo_key: new_lo_key,
        lo_count: new_lo_count,
        hi_key: new_hi_key,
        hi_count: new_hi_count,
        divide_cnt: new_divide_cnt,
    }
}

fn build_a_division_map(a: usize) -> Vec<MapEntry> {
    let mut res = Vec::new();
    let root_entry = MapEntry {
        lo_key: a,
        lo_count: 1,
        hi_key: a + 1,
        hi_count: 0,
        divide_cnt: 0,
    };
    res.push(root_entry);
    for _i in 0..29 {
        let last_entry = res.last().unwrap();
        let next_entry = divide(last_entry);
        res.push(next_entry);
    }
    res
}

fn is_ok(aa: &[usize], n: i64, m: i64, x: usize, dmap: &[Vec<MapEntry>]) -> bool {
    if x == 0 {
        return true;
    }
    let required_hi = (n + m) / 2 + 1;

    // 1. 各棒について dmap の最深有効レベル (lo_key >= x) を探す
    //    有効レベル内では「1分割 = hi +1」が常に成立する
    let mut total_cost: i64 = 0; // 全棒を最深まで分割するコスト
    let mut total_hi: i64 = 0; // 最深まで分割した時の hi 本数
    let mut lo_absorb: i64 = 0; // lo 棒の吸収容量 (各棒 v-1)
    let mut hi_values: Vec<(usize, i64)> = Vec::new(); // 犠牲用の (値, 本数)

    for (i, &a) in aa.iter().enumerate() {
        if a < x {
            if a >= 2 {
                lo_absorb += a as i64 - 1;
            }
        } else {
            // dmap[i] で lo_key >= x の最深レベルを探す (lo_key は単調減少)
            let dm = &dmap[i];
            let mut best = 0;
            for k in (0..dm.len()).rev() {
                if dm[k].lo_key >= x {
                    best = k;
                    break;
                }
            }
            let entry = &dm[best];
            total_cost += entry.divide_cnt;
            if entry.lo_count > 0 {
                total_hi += entry.lo_count;
                hi_values.push((entry.lo_key, entry.lo_count));
            }
            if entry.hi_count > 0 {
                total_hi += entry.hi_count;
                hi_values.push((entry.hi_key, entry.hi_count));
            }
        }
    }

    // v >= 2x の棒は分割すると hi が増えるので、優先する
    for i in 0..hi_values.len() {
        let (v, cnt) = hi_values[i];
        if v == 2 * x {
            total_hi += cnt;
            total_cost += cnt;
            hi_values[i] = (v / 2, cnt * 2); // 分割後の value x の棒が cnt*2 本できる
        }
    }


    // 2. m が total_cost 以下なら、m 回の分割で hi = N_hi + m
    if m <= total_cost {
        let n_hi = aa.iter().filter(|&&a| a >= x).count() as i64;
        return n_hi + m >= required_hi;
    }

    // 3. m > total_cost: 全棒を最深まで分割しても m が余る
    // このフェーズ移行 hi が増えることはあり得ないので、hi >= required_hi を満たさないなら即 NG
    if total_hi < required_hi {
        return false;
    }

    // 4. 2x - 1 -> (x - 1, x) への分割は、hiを消費せずに 分割回数 `m` を消化できるのでお得
    for i in 0..hi_values.len() {
        let (v, cnt) = hi_values[i];
        if v == 2 * x - 1 && v > 1 {
            total_cost += cnt;
            lo_absorb += cnt * (x as i64 - 2); // lo piece (x-1) の吸収容量
            hi_values[i] = (x, cnt); // 分割後の value x の棒が cnt 本できる
        }
    }

    // 無事すべての分割を消化できたので、残り m を吸収で消化できるか検査
    // ここまでの操作で hi は減少することが無いので、m 回作業できた時点で total_hi >= required_hi は保証されている
    if total_cost >= m {
        return true;
    }

    // 5. lo の範囲内で分割し続ければ、hi を減らさずに m を消化できる
    let mut remaining_m = m - total_cost;
    if remaining_m <= lo_absorb {
        return true;
    }

    // 6. ここまででも足りない場合は, 余剰の hi を犠牲にして、lo を増やす
    // hi_values を値の降順にソートし, 値の大きい棒から 長さ 1 になるまで分割する
    hi_values.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    remaining_m -= lo_absorb;
    let mut excess = total_hi - required_hi;
    for (v, cnt) in hi_values {
        if excess <= 0 || remaining_m <= 0 {
            break;
        }
        let v = v as i64;
        let absorb_per = v - 1; // 1本犠牲にすると v-1 回分の m を吸収
        if absorb_per <= 0 {
            break;
        }
        let can = cnt.min(excess);
        let need = (remaining_m + absorb_per - 1) / absorb_per;
        let actual = can.min(need);
        remaining_m -= actual * absorb_per;
        excess -= actual;
    }
    remaining_m <= 0
}

fn solve() {
    input! {
        n: usize,
        m: usize,
        aa: [usize; n],
    }

    let n_i64 = n as i64;
    let m_i64 = m as i64;
    let mut dmap = Vec::new();
    for &a in &aa {
        let d = build_a_division_map(a);
        dmap.push(d);
    }

    let mut lo = 0;
    let mut hi = 1_000_000_005;
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if is_ok(&aa, n_i64, m_i64, mid, &dmap) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    println!("{}", lo);
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        solve();
    }
}
