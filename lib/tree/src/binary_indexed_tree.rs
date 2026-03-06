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
