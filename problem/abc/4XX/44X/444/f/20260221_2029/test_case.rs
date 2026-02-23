// ===== 愚直解: DFS で全分割パターンを試して最大 median を求める =====
fn brute_force(aa: &[usize], m: usize) -> usize {
    let n = aa.len();
    let total = n + m;
    let median_idx = total / 2; // 0-indexed
    let mut best = 0;
    dfs(aa.to_vec(), m, median_idx, &mut best);
    best
}

fn dfs(sticks: Vec<usize>, remaining: usize, median_idx: usize, best: &mut usize) {
    if remaining == 0 {
        let mut sorted = sticks.clone();
        sorted.sort();
        let median = sorted[median_idx];
        if median > *best {
            *best = median;
        }
        return;
    }
    for i in 0..sticks.len() {
        if sticks[i] < 2 {
            continue;
        }
        let v = sticks[i];
        let v1 = v / 2;
        let v2 = v - v1;
        let mut new_sticks = sticks.clone();
        new_sticks.remove(i);
        new_sticks.push(v1);
        new_sticks.push(v2);
        dfs(new_sticks, remaining - 1, median_idx, best);
    }
}

// ===== 高速解 (dmap + 算術的 is_ok + 二分探索) =====

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
    for _ in 0..29 {
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

    let mut total_cost: i64 = 0;
    let mut total_hi: i64 = 0;
    let mut lo_absorb: i64 = 0;
    let mut hi_values: Vec<(usize, i64)> = Vec::new();

    for (i, &a) in aa.iter().enumerate() {
        if a < x {
            if a >= 2 {
                lo_absorb += a as i64 - 1;
            }
        } else {
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
            total_hi += entry.lo_count + entry.hi_count;
            if entry.lo_count > 0 {
                hi_values.push((entry.lo_key, entry.lo_count));
            }
            if entry.hi_count > 0 {
                hi_values.push((entry.hi_key, entry.hi_count));
            }
        }
    }

    if m <= total_cost {
        let n_hi = aa.iter().filter(|&&a| a >= x).count() as i64;
        return n_hi + m >= required_hi;
    }

    let mut remaining_m = m - total_cost;

    hi_values.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    let threshold_a = 2 * x;
    let threshold_b = 2 * x - 1;

    let mut extra_x_pieces: i64 = 0;
    for &(v, cnt) in &hi_values {
        if v < threshold_a || remaining_m <= 0 {
            break;
        }
        let splits = remaining_m.min(cnt);
        remaining_m -= splits;
        total_hi += splits;
        extra_x_pieces += splits * 2;
    }
    if remaining_m <= 0 {
        return total_hi >= required_hi;
    }

    if total_hi < required_hi {
        return false;
    }

    for &(v, cnt) in &hi_values {
        if remaining_m <= 0 {
            break;
        }
        if v != threshold_b {
            continue;
        }
        let splits = remaining_m.min(cnt);
        remaining_m -= splits;
        if x >= 2 {
            lo_absorb += splits * (x as i64 - 2);
        }
        extra_x_pieces += splits;
    }

    if remaining_m <= lo_absorb {
        return true;
    }
    remaining_m -= lo_absorb;

    let mut excess = total_hi - required_hi;
    for &(v, cnt) in &hi_values {
        if excess <= 0 || remaining_m <= 0 {
            break;
        }
        if v >= threshold_b {
            continue;
        }
        let absorb_per = v as i64 - 1;
        if absorb_per <= 0 {
            break;
        }
        let can = cnt.min(excess);
        let need = (remaining_m + absorb_per - 1) / absorb_per;
        let actual = can.min(need);
        remaining_m -= actual * absorb_per;
        excess -= actual;
    }

    if remaining_m > 0 && excess > 0 && extra_x_pieces > 0 && x >= 2 {
        let absorb_per = x as i64 - 1;
        let can = extra_x_pieces.min(excess);
        let need = (remaining_m + absorb_per - 1) / absorb_per;
        let actual = can.min(need);
        remaining_m -= actual * absorb_per;
    }

    remaining_m <= 0
}

fn fast_solve(aa: &[usize], n: usize, m: usize) -> usize {
    let n_i64 = n as i64;
    let m_i64 = m as i64;
    let dmap: Vec<Vec<MapEntry>> = aa.iter().map(|&a| build_a_division_map(a)).collect();

    let mut lo = 0;
    let mut hi = 1_000_000_005;
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if is_ok(aa, n_i64, m_i64, mid, &dmap) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo
}

fn main() {
    let mut rng: u64 = 12345;
    let mut next = || -> u64 {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        rng
    };

    let mut tested = 0;
    let mut failed = 0;

    for n in 1..=4 {
        for m in 0..=3 {
            if (n + m) % 2 == 0 {
                continue; // 問題の制約: n+m は奇数
            }
            for _ in 0..500 {
                let aa: Vec<usize> = (0..n).map(|_| (next() % 19 + 2) as usize).collect();
                let sum_a: usize = aa.iter().sum();
                if m >= sum_a || m + n > sum_a {
                    continue; // 制約: m < sum(a_i), m + n <= sum(a_i)
                }
                let expected = brute_force(&aa, m);
                let got = fast_solve(&aa, n, m);
                tested += 1;
                if expected != got {
                    failed += 1;
                    println!("FAIL: n={} m={} aa={:?}", n, m, aa);
                    println!("  expected={} got={}", expected, got);
                    if failed >= 20 {
                        println!("Too many failures, stopping.");
                        println!("Tested: {} Failed: {}", tested, failed);
                        return;
                    }
                }
            }
        }
    }
    println!("Tested: {} Failed: {}", tested, failed);
    if failed == 0 {
        println!("All tests passed!");
    }
}
