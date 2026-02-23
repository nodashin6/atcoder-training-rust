#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        aa: [i64; n],
    }
    let mut treap = Treap::new(|a, b| a + b, 0);
    for (i, &a) in aa.iter().enumerate() {
        treap.insert(i, a);
    }
    for _ in 0..q {
        input! { t: usize }
        match t {
            1 => {
                input! { l: usize }
                let l = l - 1;
                let r = l + 1;
                let l_value = treap.get(l).unwrap().clone();
                let r_value = treap.get(r).unwrap().clone();
                treap.update(l, r_value);
                treap.update(r, l_value);
            }
            2 => {
                input! { l: usize, r: usize }
                let l = l - 1;
                let r = r - 1;
                let ans = treap.prod(l, r + 1);
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}

use std::cmp::Ordering;

// ============================================================
// Treap (Key-based balanced BST with value aggregation)
// ============================================================
// insert, erase, contains, kth, count_lt, lower_bound, prod
// O(log n) expected per operation
//
// Usage:
//   let mut treap = Treap::new(|a, b| *a.min(b), i64::MAX);
//   treap.insert(3, 10);   // key=3, val=10
//   treap.insert(1, 20);
//   treap.insert(4, 5);
//   assert_eq!(treap.prod(1, 4), 10);  // min of vals with key in [1, 4) = min(20, 10) = 10
//   assert_eq!(treap.prod(1, 5), 5);   // min of vals with key in [1, 5) = min(20, 10, 5) = 5
//   treap.erase(3);

struct TreapNode<K, V: Clone> {
    key: K,
    val: V,
    acc: V,
    pri: u64,
    cnt: usize,
    left: Option<Box<TreapNode<K, V>>>,
    right: Option<Box<TreapNode<K, V>>>,
}

impl<K: Ord, V: Clone> TreapNode<K, V> {
    fn new(key: K, val: V, pri: u64) -> Self {
        let acc = val.clone();
        TreapNode {
            key,
            val,
            acc,
            pri,
            cnt: 1,
            left: None,
            right: None,
        }
    }
}

fn tp_cnt<K, V: Clone>(node: &Option<Box<TreapNode<K, V>>>) -> usize {
    node.as_ref().map_or(0, |n| n.cnt)
}

fn tp_acc<'a, K, V: Clone>(node: &'a Option<Box<TreapNode<K, V>>>, e: &'a V) -> &'a V {
    node.as_ref().map_or(e, |n| &n.acc)
}

fn tp_update<K, V: Clone>(node: &mut Box<TreapNode<K, V>>, op: fn(&V, &V) -> V, e: &V) {
    node.cnt = 1 + tp_cnt(&node.left) + tp_cnt(&node.right);
    let la = tp_acc(&node.left, e);
    let ra = tp_acc(&node.right, e);
    node.acc = (op)(&(op)(la, &node.val), ra);
}

// split: left has keys < key, right has keys >= key
fn tp_split<K: Ord, V: Clone>(
    node: Option<Box<TreapNode<K, V>>>,
    key: &K,
    op: fn(&V, &V) -> V,
    e: &V,
) -> (Option<Box<TreapNode<K, V>>>, Option<Box<TreapNode<K, V>>>) {
    match node {
        None => (None, None),
        Some(mut n) => {
            if n.key < *key {
                let (l, r) = tp_split(n.right.take(), key, op, e);
                n.right = l;
                tp_update(&mut n, op, e);
                (Some(n), r)
            } else {
                let (l, r) = tp_split(n.left.take(), key, op, e);
                n.left = r;
                tp_update(&mut n, op, e);
                (l, Some(n))
            }
        }
    }
}

// split_upper: left has keys <= key, right has keys > key
fn tp_split_upper<K: Ord, V: Clone>(
    node: Option<Box<TreapNode<K, V>>>,
    key: &K,
    op: fn(&V, &V) -> V,
    e: &V,
) -> (Option<Box<TreapNode<K, V>>>, Option<Box<TreapNode<K, V>>>) {
    match node {
        None => (None, None),
        Some(mut n) => {
            if n.key <= *key {
                let (l, r) = tp_split_upper(n.right.take(), key, op, e);
                n.right = l;
                tp_update(&mut n, op, e);
                (Some(n), r)
            } else {
                let (l, r) = tp_split_upper(n.left.take(), key, op, e);
                n.left = r;
                tp_update(&mut n, op, e);
                (l, Some(n))
            }
        }
    }
}

fn tp_merge<K: Ord, V: Clone>(
    left: Option<Box<TreapNode<K, V>>>,
    right: Option<Box<TreapNode<K, V>>>,
    op: fn(&V, &V) -> V,
    e: &V,
) -> Option<Box<TreapNode<K, V>>> {
    match (left, right) {
        (None, r) => r,
        (l, None) => l,
        (Some(mut l), Some(mut r)) => {
            if l.pri > r.pri {
                l.right = tp_merge(l.right.take(), Some(r), op, e);
                tp_update(&mut l, op, e);
                Some(l)
            } else {
                r.left = tp_merge(Some(l), r.left.take(), op, e);
                tp_update(&mut r, op, e);
                Some(r)
            }
        }
    }
}

// Binary search on BST: find the first key (left to right) where f(running_acc) becomes false.
// Returns (accumulated_value, Some(key)) if found, (accumulated_value, None) if all satisfy.
fn tp_max_right<K: Ord + Clone, V: Clone>(
    node: &Option<Box<TreapNode<K, V>>>,
    acc: V,
    op: fn(&V, &V) -> V,
    e: &V,
    f: &impl Fn(&V) -> bool,
) -> (V, Option<K>) {
    match node {
        None => (acc, None),
        Some(n) => {
            let left_acc = (op)(&acc, tp_acc(&n.left, e));
            let with_cur = (op)(&left_acc, &n.val);
            if f(&with_cur) {
                tp_max_right(&n.right, with_cur, op, e, f)
            } else if f(&left_acc) {
                (left_acc, Some(n.key.clone()))
            } else {
                tp_max_right(&n.left, acc, op, e, f)
            }
        }
    }
}

// Binary search on BST (right to left): find the first key where f(running_acc) becomes false.
fn tp_min_left<K: Ord + Clone, V: Clone>(
    node: &Option<Box<TreapNode<K, V>>>,
    acc: V,
    op: fn(&V, &V) -> V,
    e: &V,
    f: &impl Fn(&V) -> bool,
) -> (V, Option<K>) {
    match node {
        None => (acc, None),
        Some(n) => {
            let right_acc = (op)(tp_acc(&n.right, e), &acc);
            let with_cur = (op)(&n.val, &right_acc);
            if f(&with_cur) {
                tp_min_left(&n.left, with_cur, op, e, f)
            } else if f(&right_acc) {
                (right_acc, Some(n.key.clone()))
            } else {
                tp_min_left(&n.right, acc, op, e, f)
            }
        }
    }
}

fn tp_set_val<K: Ord, V: Clone>(
    node: &mut Option<Box<TreapNode<K, V>>>,
    key: &K,
    val: V,
    op: fn(&V, &V) -> V,
    e: &V,
) -> bool {
    match node {
        None => false,
        Some(n) => {
            let found = match key.cmp(&n.key) {
                Ordering::Less => tp_set_val(&mut n.left, key, val, op, e),
                Ordering::Equal => {
                    n.val = val;
                    true
                }
                Ordering::Greater => tp_set_val(&mut n.right, key, val, op, e),
            };
            if found {
                tp_update(n, op, e);
            }
            found
        }
    }
}

fn tp_kth<K, V: Clone>(node: &Option<Box<TreapNode<K, V>>>, k: usize) -> Option<(&K, &V)> {
    let n = node.as_ref()?;
    let lc = tp_cnt(&n.left);
    match k.cmp(&lc) {
        Ordering::Less => tp_kth(&n.left, k),
        Ordering::Equal => Some((&n.key, &n.val)),
        Ordering::Greater => tp_kth(&n.right, k - lc - 1),
    }
}

pub struct Treap<K, V: Clone> {
    root: Option<Box<TreapNode<K, V>>>,
    op: fn(&V, &V) -> V,
    e: V,
    rng: XorShift,
}

impl<K: Ord, V: Clone> Treap<K, V> {
    pub fn new(op: fn(&V, &V) -> V, e: V) -> Self {
        Treap {
            root: None,
            op,
            e,
            rng: XorShift::new(),
        }
    }

    pub fn len(&self) -> usize {
        tp_cnt(&self.root)
    }

    /// Update the value for an existing key. Returns true if found.
    pub fn update(&mut self, key: K, val: V) -> bool {
        tp_set_val(&mut self.root, &key, val, self.op, &self.e)
    }

    /// Insert (key, val). If key already exists, overwrite val.
    pub fn insert(&mut self, key: K, val: V) {
        let pri = self.rng.next();
        let new_node = Some(Box::new(TreapNode::new(key, val, pri)));
        let k = &new_node.as_ref().unwrap().key;
        let root = self.root.take();
        let (l, mr) = tp_split(root, k, self.op, &self.e);
        let (_, r) = tp_split_upper(mr, k, self.op, &self.e);
        self.root = tp_merge(tp_merge(l, new_node, self.op, &self.e), r, self.op, &self.e);
    }

    /// Erase the element with the given key. Returns true if found.
    pub fn erase(&mut self, key: K) -> bool {
        let root = self.root.take();
        let (l, mr) = tp_split(root, &key, self.op, &self.e);
        let (m, r) = tp_split_upper(mr, &key, self.op, &self.e);
        if m.is_none() {
            self.root = tp_merge(l, r, self.op, &self.e);
            return false;
        }
        let m_node = m.unwrap();
        let rest = tp_merge(m_node.left, m_node.right, self.op, &self.e);
        self.root = tp_merge(tp_merge(l, rest, self.op, &self.e), r, self.op, &self.e);
        true
    }

    /// Get a reference to the value for the given key.
    pub fn get(&self, key: K) -> Option<&V> {
        let mut cur = &self.root;
        while let Some(n) = cur {
            match key.cmp(&n.key) {
                Ordering::Less => cur = &n.left,
                Ordering::Equal => return Some(&n.val),
                Ordering::Greater => cur = &n.right,
            }
        }
        None
    }

    pub fn contains(&self, key: K) -> bool {
        let mut cur = &self.root;
        while let Some(n) = cur {
            match key.cmp(&n.key) {
                Ordering::Less => cur = &n.left,
                Ordering::Equal => return true,
                Ordering::Greater => cur = &n.right,
            }
        }
        false
    }

    /// k-th smallest (key, val) pair (0-indexed)
    pub fn kth(&self, k: usize) -> Option<(&K, &V)> {
        tp_kth(&self.root, k)
    }

    /// Number of elements with key strictly less than given key
    pub fn count_lt(&self, key: K) -> usize {
        let mut cur = &self.root;
        let mut cnt = 0;
        while let Some(n) = cur {
            if n.key < key {
                cnt += 1 + tp_cnt(&n.left);
                cur = &n.right;
            } else {
                cur = &n.left;
            }
        }
        cnt
    }

    /// Number of elements with key <= given key
    pub fn count_le(&self, key: K) -> usize {
        let mut cur = &self.root;
        let mut cnt = 0;
        while let Some(n) = cur {
            if n.key <= key {
                cnt += 1 + tp_cnt(&n.left);
                cur = &n.right;
            } else {
                cur = &n.left;
            }
        }
        cnt
    }

    /// Smallest key in the treap
    pub fn key_min(&self) -> Option<&K> {
        let mut cur = &self.root;
        let mut res: Option<&K> = None;
        while let Some(n) = cur {
            res = Some(&n.key);
            cur = &n.left;
        }
        res
    }

    /// Largest key in the treap
    pub fn key_max(&self) -> Option<&K> {
        let mut cur = &self.root;
        let mut res: Option<&K> = None;
        while let Some(n) = cur {
            res = Some(&n.key);
            cur = &n.right;
        }
        res
    }

    /// Smallest key >= given key
    pub fn lower_bound(&self, key: K) -> Option<&K> {
        let mut cur = &self.root;
        let mut res: Option<&K> = None;
        while let Some(n) = cur {
            if n.key >= key {
                res = Some(&n.key);
                cur = &n.left;
            } else {
                cur = &n.right;
            }
        }
        res
    }

    /// Smallest key > given key
    pub fn upper_bound(&self, key: K) -> Option<&K> {
        let mut cur = &self.root;
        let mut res: Option<&K> = None;
        while let Some(n) = cur {
            if n.key > key {
                res = Some(&n.key);
                cur = &n.left;
            } else {
                cur = &n.right;
            }
        }
        res
    }

    /// Aggregate values for keys in [lo, hi) using op
    pub fn prod(&mut self, lo: K, hi: K) -> V {
        let root = self.root.take();
        let (left, mr) = tp_split(root, &lo, self.op, &self.e);
        let (mid, right) = tp_split(mr, &hi, self.op, &self.e);
        let res = mid.as_ref().map_or(self.e.clone(), |m| m.acc.clone());
        self.root = tp_merge(
            tp_merge(left, mid, self.op, &self.e),
            right,
            self.op,
            &self.e,
        );
        res
    }

    /// Starting from key l, find the first key k (>= l) where
    /// f(aggregate of values for keys in [l, k]) becomes false.
    /// Returns Some(k) if found, None if all keys >= l satisfy f.
    /// Precondition: f(e) is true.
    pub fn max_right(&mut self, l: K, f: impl Fn(&V) -> bool) -> Option<K>
    where
        K: Clone,
    {
        let root = self.root.take();
        let (left, right) = tp_split(root, &l, self.op, &self.e);
        let (_, result) = tp_max_right(&right, self.e.clone(), self.op, &self.e, &f);
        self.root = tp_merge(left, right, self.op, &self.e);
        result
    }

    /// Starting from the largest key < r, going left, find the first key k
    /// where f(aggregate of values for keys in [k, r)) becomes false.
    /// Returns Some(k) if found, None if all keys < r satisfy f.
    /// Precondition: f(e) is true.
    pub fn min_left(&mut self, r: K, f: impl Fn(&V) -> bool) -> Option<K>
    where
        K: Clone,
    {
        let root = self.root.take();
        let (left, right) = tp_split(root, &r, self.op, &self.e);
        let (_, result) = tp_min_left(&left, self.e.clone(), self.op, &self.e, &f);
        self.root = tp_merge(left, right, self.op, &self.e);
        result
    }
}

struct XorShift {
    state: u64,
}

impl XorShift {
    fn new() -> Self {
        XorShift {
            state: 88172645463325252,
        }
    }

    fn next(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }
}
