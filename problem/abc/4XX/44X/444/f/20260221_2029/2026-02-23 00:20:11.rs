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

fn is_ok(aa: &[usize], n: i64, mut m: i64, x: usize, _sum_a: i64, dmap: &[Vec<MapEntry>]) -> bool {
    let mut lo_treap: Treap<usize, i64> = Treap::new(|a, b| *a + *b, 0); // x未満の棒を管理するTreap
    let mut hi_treap: Treap<usize, i64> = Treap::new(|a, b| *a + *b, 0); // x以上の棒を管理するTreap
    let threshold_x = 2 * x - 1; // 2x - 1以上の棒は分割して損することはない
    let required_hi = (n + m) / 2 + 1;

    lo_treap.insert(1, 0); // 長さ1の棒が0本ある
    for (i, &a) in aa.iter().enumerate() {
        if a < x {
            let a_count = lo_treap.prod(a, a + 1);
            lo_treap.insert(a, a_count + 1);
        } else {
            let res = &dmap[i];
            for entry in res.iter().rev() {
                if x <= entry.lo_key && entry.divide_cnt <= m {
                    let a_count = hi_treap.prod(entry.lo_key, entry.lo_key + 1);
                    hi_treap.insert(entry.lo_key, a_count + entry.lo_count);
                    let a_count = hi_treap.prod(entry.hi_key, entry.hi_key + 1);
                    hi_treap.insert(entry.hi_key, a_count + entry.hi_count);
                    m -= entry.divide_cnt;
                    break;
                }
            }
        }
    }

    // まずは (2x - 1) 以上の棒を分割していく
    while m > 0 {
        let hi_max = match hi_treap.key_max() {
            Some(&k) if k >= threshold_x => k,
            _ => break,
        };
        let count = hi_treap.prod(hi_max, hi_max + 1);
        let div_count = m.min(count);
        m -= div_count;
        if count == div_count {
            hi_treap.erase(hi_max);
        } else {
            hi_treap.insert(hi_max, count - div_count);
        }
        if threshold_x < hi_max {
            if hi_max % 2 == 0 {
                let v = hi_max / 2;
                let v_count = hi_treap.prod(v, v + 1) + div_count * 2;
                hi_treap.insert(v, v_count);
            } else {
                let v1 = hi_max / 2;
                let v1_count = hi_treap.prod(v1, v1 + 1) + div_count;
                hi_treap.insert(v1, v1_count);
                let v2 = hi_max - v1;
                let v2_count = hi_treap.prod(v2, v2 + 1) + div_count;
                hi_treap.insert(v2, v2_count);
            }
        } else if hi_max == threshold_x {
            // hi_max は奇数
            let v_lo = hi_max / 2;
            let v_hi = hi_max - v_lo;
            let v_lo_count = lo_treap.prod(v_lo, v_lo + 1) + div_count;
            lo_treap.insert(v_lo, v_lo_count);
            let v_hi_count = hi_treap.prod(v_hi, v_hi + 1) + div_count;
            hi_treap.insert(v_hi, v_hi_count);
        }
    }

    // この時点で hi_treap に不足がある --> NG が確定する
    if hi_treap.prod(0, usize::MAX) < required_hi {
        return false;
    } else if m == 0 {
        // 数量をみたしていて、m がゼロ --> OK が確定する
        return true;
    }

    // hi に required_hi 以上存在する必要はないので、
    // (n + m) / 2 + 1 以上を残しつつ、 hi の最大値を分割していく
    // すでにこの時点で hi側は 2x - 1 未満になっているので、
    // 分割した両方が lo_treap に入ることになる
    while m > 0 && hi_treap.prod(0, usize::MAX) > required_hi {
        let hi_max = hi_treap.key_max().unwrap().clone();
        let count = hi_treap.prod(hi_max, hi_max + 1);
        let excess_hi = hi_treap.prod(0, usize::MAX) - required_hi;
        let div_count = m.min(count.min(excess_hi));
        m -= div_count;
        if count == div_count {
            hi_treap.erase(hi_max);
        } else {
            hi_treap.insert(hi_max, count - div_count);
        };
        let v_lo = hi_max / 2;
        let v_hi = hi_max - v_lo;
        let v_lo_count = lo_treap.prod(v_lo, v_lo + 1) + div_count;
        lo_treap.insert(v_lo, v_lo_count);
        let v_hi_count = lo_treap.prod(v_hi, v_hi + 1) + div_count;
        lo_treap.insert(v_hi, v_hi_count);
    }

    // この時点で hi_treap に不足がある --> NG が確定する
    if hi_treap.prod(0, usize::MAX) < required_hi {
        return false;
    } else if m == 0 {
        // 数量をみたしていて、m がゼロ --> OK が確定する
        return true;
    }

    // lo の本数が足りているか確認
    // lo の方は、長さ1ずつ分割できるとみなしても問題の性質を損なわない
    let mut lo_all = lo_treap.prod(0, usize::MAX);
    while m > 0 && lo_all < required_hi {
        let lo_max = match lo_treap.key_max() {
            Some(&k) if k > 1 => k,
            _ => break,
        };
        let count = lo_treap.prod(lo_max, lo_max + 1);
        let can_div_count = (lo_max as i64 - 1) * count;
        let div_count = m.min((required_hi - lo_all).min(can_div_count));
        m -= div_count;
        if count == div_count {
            lo_treap.erase(lo_max);
        } else {
            lo_treap.insert(lo_max, count - div_count); // 長さ lo_max の棒が減る
            let count_1 = lo_treap.prod(1, 2);
            lo_treap.insert(1, count_1 + div_count); // 長さ1の棒が増える
            lo_all += div_count;
        }
    }

    // lo 側が足りていれば、この時点で OK が確定する
    if m == 0 {
        return true;
    }

    let result = m == 0;
    return result;
}

fn solve() {
    input! {
        n: usize,
        m: usize,
        aa: [usize; n],
    }

    let sum_a: i64 = aa.iter().map(|&a| a as i64).sum();
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
        if is_ok(&aa, n_i64, m_i64, mid, sum_a, &dmap) {
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
