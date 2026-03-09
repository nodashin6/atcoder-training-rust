#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut aa: [i64; n],
        xy: [(i64, i64); q],
    }
    aa.sort();
    aa.dedup();
    let count_le = |val: i64| aa.partition_point(|&x| x <= val);
    for (x, y) in xy {
        let offset = count_le(x - 1);
        let mut lo = x - 1 as i64;
        let mut hi = x + y + (n + 5) as i64;
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            // length: x, x + 1, ..., mid の個数
            let length = mid - x + 1;
            // treap に存在する x 以上 mid 以下の個数
            let exists = (count_le(mid) as i64 - offset as i64).max(0);
            // treap に存在しない x以上 mid 以下の個数が y 個以下なら、mid は答えの候補
            let not_exists = length - exists;
            if not_exists < y {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        println!("{}", hi);
    }
}
