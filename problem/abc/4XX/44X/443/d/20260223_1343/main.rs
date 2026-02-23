use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn solve() {
    input! {
        n: usize,
        mut rr: [usize; n],
    }
    let mut ans = 0;
    let mut heap = BinaryHeap::new();
    for i in 0..n  {
        heap.push(Reverse((rr[i], i)));
    }
    while heap.len() > 1 {
        let Reverse((r, i)) = heap.pop().unwrap();
        if rr[i] != r {
            continue;
        }
        if i > 0 && rr[i - 1] > rr[i] + 1 {
            ans += rr[i - 1] - (rr[i] + 1);
            rr[i - 1] = rr[i] + 1;
            heap.push(Reverse((rr[i - 1], i - 1)));
        }
        if i + 1 < n && rr[i + 1] > rr[i] + 1 {
            ans += rr[i + 1] - (rr[i] + 1);
            rr[i + 1] = rr[i] + 1;
            heap.push(Reverse((rr[i + 1], i + 1)));
        }
    }
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
