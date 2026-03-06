#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let mut aa = vec![];
    for c in s.chars() {
        if c == 'A' {
            aa.push(1i64)
        } else if c == 'B' {
            aa.push(-1)
        } else {
            aa.push(0)
        }
    }
    let mut bb = vec![0i64; n + 1];
    for i in 0..n {
        bb[i + 1] = bb[i] + aa[i];
    }
    let offset = n as i64;
    let sz = 2 * n + 1;
    let mut bit = BinaryIndexedTree::new(sz);

    for i in 1..=n {
        let idx = (bb[i] + offset) as usize;
        bit.add(idx, 1);
    }

    let mut ans = 0i64;
    for i in 0..n {
        let idx = (bb[i] + offset) as usize;
        let total = (n - i) as i64;
        let le = bit.sum(idx);
        ans += total - le;
        let ridx = (bb[i + 1] + offset) as usize;
        bit.add(ridx, -1);
    }
    println!("{}", ans);
}

pub struct BinaryIndexedTree {
    n: usize,
    data: Vec<i64>,
}

impl BinaryIndexedTree {
    pub fn new(n: usize) -> Self {
        BinaryIndexedTree {
            n,
            data: vec![0; n + 1],
        }
    }

    pub fn add(&mut self, i: usize, v: i64) {
        let mut i = i + 1;
        while i <= self.n {
            self.data[i] += v;
            i += i & i.wrapping_neg();
        }
    }

    /// Sum of [0, i] (inclusive)
    pub fn sum(&self, i: usize) -> i64 {
        let mut i = i + 1;
        let mut s = 0i64;
        while i > 0 {
            s += self.data[i];
            i -= i & i.wrapping_neg();
        }
        s
    }

    /// Sum of [l, r) (half-open)
    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        if l >= r {
            return 0;
        }
        if l == 0 {
            self.sum(r - 1)
        } else {
            self.sum(r - 1) - self.sum(l - 1)
        }
    }
}
