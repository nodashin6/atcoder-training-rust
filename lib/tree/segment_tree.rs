struct Segtree<T: Clone> {
    n: usize,
    op: fn(&T, &T) -> T,
    e: T,
    a: Vec<T>,
}

impl<T: Clone> Segtree<T> {
    fn new(n: usize, op: fn(&T, &T) -> T, e: T, a: Vec<T>) -> Self {
        let n_new = n.next_power_of_two();
        let a_new = vec![e.clone(); n_new * 2];
        let mut segtree = Segtree {
            n: n_new,
            op,
            e,
            a: a_new,
        };
        for i in 0..n {
            segtree.set(i, a[i].clone());
        }
        segtree
    }

    fn set(&mut self, p: usize, x: T) {
        let mut p = p + self.n;
        self.a[p] = x;
        while p > 1 {
            p >>= 1;
            self.a[p] = (self.op)(&self.a[p << 1], &self.a[p << 1 | 1]);
        }
    }

    fn get(&self, p: usize) -> T {
        self.a[p + self.n].clone()
    }

    fn prod(&self, l: usize, r: usize) -> T {
        let mut l = l + self.n;
        let mut r = r + self.n;
        let mut sml = self.e.clone();
        let mut smr = self.e.clone();
        while l < r {
            if l & 1 == 1 {
                sml = (self.op)(&sml, &self.a[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = (self.op)(&self.a[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(&sml, &smr)
    }

    fn min_left<F: Fn(T) -> bool>(&self, l: usize, f: F) -> usize {
        if l == self.n {
            return self.n;
        }
        let mut l = l + self.n;
        let mut sm = self.e.clone();
        loop {
            while l % 2 == 0 {
                l >>= 1;
            }
            if !f((self.op)(&sm, &self.a[l])) {
                while l < self.n {
                    l <<= 1;
                    if f((self.op)(&sm, &self.a[l])) {
                        sm = (self.op)(&sm, &self.a[l]);
                        l += 1;
                    }
                }
                return l - self.n;
            }
            sm = (self.op)(&sm, &self.a[l]);
            l += 1;
            if (l & l.wrapping_neg()) == l {
                break;
            }
        }
        return self.n;
    }

    fn max_right<F: Fn(T) -> bool>(&self, r: usize, f: F) -> usize {
        if r == 0 {
            return 0;
        }
        let mut r = r + self.n;
        let mut sm = self.e.clone();
        loop {
            r -= 1;
            while r > 1 && r % 2 == 1 {
                r >>= 1;
            }
            if !f((self.op)(&sm, &self.a[r])) {
                while r < self.n {
                    r <<= 1;
                    if f((self.op)(&sm, &self.a[r])) {
                        sm = (self.op)(&sm, &self.a[r]);
                        r += 1;
                    }
                }
                return r - self.n;
            }
            sm = (self.op)(&sm, &self.a[r]);
            if (r & r.wrapping_neg()) == r {
                break;
            }
        }
        return 0;
    }
}
