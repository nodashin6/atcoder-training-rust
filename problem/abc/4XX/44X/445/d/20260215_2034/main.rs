use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        shapes: [(usize, usize); n],
    }

    let mut x: Vec<usize> = vec![0; n];
    let mut y: Vec<usize> = vec![0; n];
    for i in 0..n {
        x[i] = shapes[i].0;
        y[i] = shapes[i].1;
    }
    let mut x_segtree = Segtree::new(n, max, 0, x);
    let mut y_segtree = Segtree::new(n, max, 0, y);
    let mut current_x = h;
    let mut current_y = w;
    let mut ans = vec![(0, 0); n];
    while current_x > 0 && current_y > 0 {
        let max_x = x_segtree.prod(0, n);
        let max_y = y_segtree.prod(0, n);
        if max_x == current_x && max_y == current_y {
            let i = x_segtree.min_left(0, |a| a < current_x);
            current_x = 0;
            current_y = 0;
            ans[i] = (0, 0);
        } else if max_x == current_x {
            let i = x_segtree.min_left(0, |a| a < current_x);
            let x = max_x;
            let y = shapes[i].1;
            current_y = current_y - y;
            ans[i] = (current_x - x, current_y);
            x_segtree.set(i, 0);
            y_segtree.set(i, 0);
        } else if max_y == current_y {
            let i = y_segtree.min_left(0, |a| a < current_y);
            let x = shapes[i].0;
            let y = max_y;
            current_x = current_x - x;
            ans[i] = (current_x, current_y - y);
            x_segtree.set(i, 0);
            y_segtree.set(i, 0);
        }
    }

    for (x, y) in ans {
        println!("{} {}", x + 1, y + 1);
    }
}

struct Segtree<T> {
    n: usize,
    op: fn(T, T) -> T,
    e: T,
    a: Vec<T>,
}

impl<T: Copy> Segtree<T> {
    fn new(n: usize, op: fn(T, T) -> T, e: T, a: Vec<T>) -> Self {
        let n_new = n.next_power_of_two();
        let a_new = vec![e; n_new * 2];
        let mut segtree = Segtree {
            n: n_new,
            op,
            e,
            a: a_new,
        };
        for i in 0..n {
            segtree.set(i, a[i]);
        }
        segtree
    }

    fn set(&mut self, p: usize, x: T) {
        let mut p = p + self.n;
        self.a[p] = x;
        while p > 1 {
            p >>= 1;
            self.a[p] = (self.op)(self.a[p << 1], self.a[p << 1 | 1]);
        }
    }

    fn get(&self, p: usize) -> T {
        self.a[p + self.n]
    }

    fn prod(&self, l: usize, r: usize) -> T {
        let mut l = l + self.n;
        let mut r = r + self.n;
        let mut sml = self.e;
        let mut smr = self.e;
        while l < r {
            if l & 1 == 1 {
                sml = (self.op)(sml, self.a[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = (self.op)(self.a[r], smr);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(sml, smr)
    }

    fn min_left<F: Fn(T) -> bool>(&self, l: usize, f: F) -> usize {
        if l == self.n {
            return self.n;
        }
        let mut l = l + self.n;
        let mut sm = self.e;
        loop {
            while l % 2 == 0 {
                l >>= 1;
            }
            if !f((self.op)(sm, self.a[l])) {
                while l < self.n {
                    l <<= 1;
                    if f((self.op)(sm, self.a[l])) {
                        sm = (self.op)(sm, self.a[l]);
                        l += 1;
                    }
                }
                return l - self.n;
            }
            sm = (self.op)(sm, self.a[l]);
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
        let mut sm = self.e;
        loop {
            r -= 1;
            while r > 1 && r % 2 == 1 {
                r >>= 1;
            }
            if !f((self.op)(sm, self.a[r])) {
                while r < self.n {
                    r <<= 1;
                    if f((self.op)(sm, self.a[r])) {
                        sm = (self.op)(sm, self.a[r]);
                        r += 1;
                    }
                }
                return r - self.n;
            }
            sm = (self.op)(sm, self.a[r]);
            if (r & r.wrapping_neg()) == r {
                break;
            }
        }
        return 0;
    }
}
