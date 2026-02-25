#[allow(unused_imports)]
use itertools::Itertools;
use num_rational::Ratio;
use proconio::input;
use std::collections::BTreeMap;

fn calc_degree(x: i32, y: i32) -> (usize, Ratio<i32>) {
    if 0 <= x && 0 < y {
        (0, Ratio::new(x, y))
    } else if 0 < x && y <= 0 {
        (1, Ratio::new(-y, x))
    } else if x <= 0 && y < 0 {
        (2, Ratio::new(-x, -y))
    } else {
        (3, Ratio::new(y, -x))
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(i32, i32); n],
        ab: [(usize, usize); q],
    }

    let mut degree_unique: BTreeMap<(usize, Ratio<i32>), Vec<usize>> = BTreeMap::new();
    for (i, &(x, y)) in xy.iter().enumerate() {
        let (bucket, deg) = calc_degree(x, y);
        degree_unique.entry((bucket, deg)).or_default().push(i);
    }

    // もうdgreeである必要はないので, sortして pos <usize> に変換する
    // 円状のリストは、2周することで、0 <= pos < m と m <= pos < 2m の両方を考慮できるようにする
    let m = degree_unique.len();
    let mut two_pos = vec![vec![0, 0]; n];
    let mut dd = vec![0; 2 * m];
    for (pos, key) in degree_unique.keys().enumerate() {
        for index in degree_unique[key].iter() {
            two_pos[*index][0] = pos;
            two_pos[*index][1] = pos + m;
            dd[pos] += 1;
            dd[pos + m] += 1;
        }
    }

    let mut acc = vec![0; 2 * m + 1];
    for i in 0..2 * m {
        acc[i + 1] = acc[i] + dd[i];
    }
    for (mut a, mut b) in ab {
        a -= 1;
        b -= 1;

        let a_pos = two_pos[a][0];
        let b_first = two_pos[b][0];
        let b_pos = if b_first > a_pos {
            b_first
        } else if b_first == a_pos {
            // 同じ方角だと回転しない
            a_pos
        } else {
            // b が a より手前なので、1周回る
            two_pos[b][1]
        };
        println!("{}", acc[b_pos + 1] - acc[a_pos]);
    }
}
