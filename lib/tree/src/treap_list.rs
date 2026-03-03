use std::cmp::Ordering;

// ============================================================
// TreapList (Multiset with value aggregation)
// ============================================================
// insert, erase, contains, count, kth, count_lt, lower_bound, prod
// O(log n) expected per operation
// Duplicates are allowed.
//
// Usage:
//   let mut tl = TreapList::new(|a, b| a + b, 0i64);
//   tl.insert(3);
//   tl.insert(1);
//   tl.insert(3);  // duplicate allowed
//   assert_eq!(tl.len(), 3);
//   assert_eq!(tl.count(3), 2);
//   assert_eq!(tl.kth(0), Some(&1));
//   assert_eq!(tl.prod(0, 3), 7);  // 1 + 3 + 3 (0-indexed)
//   tl.erase(3);  // removes one occurrence
//   assert_eq!(tl.count(3), 1);

struct TreapListNode<V: Clone> {
    val: V,
    acc: V,
    pri: u64,
    cnt: usize,
    left: Option<Box<TreapListNode<V>>>,
    right: Option<Box<TreapListNode<V>>>,
}

impl<V: Clone + Ord> TreapListNode<V> {
    fn new(val: V, pri: u64) -> Self {
        let acc = val.clone();
        TreapListNode {
            val,
            acc,
            pri,
            cnt: 1,
            left: None,
            right: None,
        }
    }
}

fn tl_cnt<V: Clone>(node: &Option<Box<TreapListNode<V>>>) -> usize {
    node.as_ref().map_or(0, |n| n.cnt)
}

fn tl_acc<'a, V: Clone>(node: &'a Option<Box<TreapListNode<V>>>, e: &'a V) -> &'a V {
    node.as_ref().map_or(e, |n| &n.acc)
}

fn tl_update<V: Clone>(node: &mut Box<TreapListNode<V>>, op: fn(&V, &V) -> V, e: &V) {
    node.cnt = 1 + tl_cnt(&node.left) + tl_cnt(&node.right);
    let la = tl_acc(&node.left, e);
    let ra = tl_acc(&node.right, e);
    node.acc = (op)(&(op)(la, &node.val), ra);
}

// split by value: left has vals < key, right has vals >= key
fn tl_split<V: Clone + Ord>(
    node: Option<Box<TreapListNode<V>>>,
    key: &V,
    op: fn(&V, &V) -> V,
    e: &V,
) -> (Option<Box<TreapListNode<V>>>, Option<Box<TreapListNode<V>>>) {
    match node {
        None => (None, None),
        Some(mut n) => {
            if n.val < *key {
                let (l, r) = tl_split(n.right.take(), key, op, e);
                n.right = l;
                tl_update(&mut n, op, e);
                (Some(n), r)
            } else {
                let (l, r) = tl_split(n.left.take(), key, op, e);
                n.left = r;
                tl_update(&mut n, op, e);
                (l, Some(n))
            }
        }
    }
}

// split_upper by value: left has vals <= key, right has vals > key
fn tl_split_upper<V: Clone + Ord>(
    node: Option<Box<TreapListNode<V>>>,
    key: &V,
    op: fn(&V, &V) -> V,
    e: &V,
) -> (Option<Box<TreapListNode<V>>>, Option<Box<TreapListNode<V>>>) {
    match node {
        None => (None, None),
        Some(mut n) => {
            if n.val <= *key {
                let (l, r) = tl_split_upper(n.right.take(), key, op, e);
                n.right = l;
                tl_update(&mut n, op, e);
                (Some(n), r)
            } else {
                let (l, r) = tl_split_upper(n.left.take(), key, op, e);
                n.left = r;
                tl_update(&mut n, op, e);
                (l, Some(n))
            }
        }
    }
}

// split by rank: left has first k elements, right has the rest
fn tl_split_at<V: Clone + Ord>(
    node: Option<Box<TreapListNode<V>>>,
    k: usize,
    op: fn(&V, &V) -> V,
    e: &V,
) -> (Option<Box<TreapListNode<V>>>, Option<Box<TreapListNode<V>>>) {
    match node {
        None => (None, None),
        Some(mut n) => {
            let lc = tl_cnt(&n.left);
            if k <= lc {
                let (l, r) = tl_split_at(n.left.take(), k, op, e);
                n.left = r;
                tl_update(&mut n, op, e);
                (l, Some(n))
            } else {
                let (l, r) = tl_split_at(n.right.take(), k - lc - 1, op, e);
                n.right = l;
                tl_update(&mut n, op, e);
                (Some(n), r)
            }
        }
    }
}

fn tl_merge<V: Clone + Ord>(
    left: Option<Box<TreapListNode<V>>>,
    right: Option<Box<TreapListNode<V>>>,
    op: fn(&V, &V) -> V,
    e: &V,
) -> Option<Box<TreapListNode<V>>> {
    match (left, right) {
        (None, r) => r,
        (l, None) => l,
        (Some(mut l), Some(mut r)) => {
            if l.pri > r.pri {
                l.right = tl_merge(l.right.take(), Some(r), op, e);
                tl_update(&mut l, op, e);
                Some(l)
            } else {
                r.left = tl_merge(Some(l), r.left.take(), op, e);
                tl_update(&mut r, op, e);
                Some(r)
            }
        }
    }
}

// Binary search on tree (left to right): returns (acc, count of elements satisfying f)
fn tl_max_right<V: Clone>(
    node: &Option<Box<TreapListNode<V>>>,
    acc: V,
    op: fn(&V, &V) -> V,
    e: &V,
    f: &impl Fn(&V) -> bool,
) -> (V, usize) {
    match node {
        None => (acc, 0),
        Some(n) => {
            let left_acc = (op)(&acc, tl_acc(&n.left, e));
            let with_cur = (op)(&left_acc, &n.val);
            if f(&with_cur) {
                let (racc, ridx) = tl_max_right(&n.right, with_cur, op, e, f);
                (racc, tl_cnt(&n.left) + 1 + ridx)
            } else if f(&left_acc) {
                (left_acc, tl_cnt(&n.left))
            } else {
                tl_max_right(&n.left, acc, op, e, f)
            }
        }
    }
}

// Binary search on tree (right to left): returns (acc, count of elements satisfying f)
fn tl_min_left<V: Clone>(
    node: &Option<Box<TreapListNode<V>>>,
    acc: V,
    op: fn(&V, &V) -> V,
    e: &V,
    f: &impl Fn(&V) -> bool,
) -> (V, usize) {
    match node {
        None => (acc, 0),
        Some(n) => {
            let right_acc = (op)(tl_acc(&n.right, e), &acc);
            let with_cur = (op)(&n.val, &right_acc);
            if f(&with_cur) {
                let (lacc, lidx) = tl_min_left(&n.left, with_cur, op, e, f);
                (lacc, tl_cnt(&n.right) + 1 + lidx)
            } else if f(&right_acc) {
                (right_acc, tl_cnt(&n.right))
            } else {
                tl_min_left(&n.right, acc, op, e, f)
            }
        }
    }
}

fn tl_kth<V: Clone>(node: &Option<Box<TreapListNode<V>>>, k: usize) -> Option<&V> {
    let n = node.as_ref()?;
    let lc = tl_cnt(&n.left);
    match k.cmp(&lc) {
        Ordering::Less => tl_kth(&n.left, k),
        Ordering::Equal => Some(&n.val),
        Ordering::Greater => tl_kth(&n.right, k - lc - 1),
    }
}

pub struct TreapList<V: Clone> {
    root: Option<Box<TreapListNode<V>>>,
    op: fn(&V, &V) -> V,
    e: V,
    rng: XorShift,
}

impl<V: Clone + Ord> TreapList<V> {
    pub fn new(op: fn(&V, &V) -> V, e: V) -> Self {
        TreapList {
            root: None,
            op,
            e,
            rng: XorShift::new(),
        }
    }

    pub fn len(&self) -> usize {
        tl_cnt(&self.root)
    }

    /// Insert val. Duplicates are allowed.
    pub fn insert(&mut self, val: V) {
        let pri = self.rng.next();
        let new_node = Some(Box::new(TreapListNode::new(val, pri)));
        let k = &new_node.as_ref().unwrap().val;
        let root = self.root.take();
        let (l, r) = tl_split(root, k, self.op, &self.e);
        self.root = tl_merge(tl_merge(l, new_node, self.op, &self.e), r, self.op, &self.e);
    }

    /// Remove one occurrence of val. Returns true if found.
    pub fn erase(&mut self, val: V) -> bool {
        let root = self.root.take();
        let (l, mr) = tl_split(root, &val, self.op, &self.e);
        let (m, r) = tl_split_upper(mr, &val, self.op, &self.e);
        match m {
            None => {
                self.root = tl_merge(l, r, self.op, &self.e);
                false
            }
            Some(m_node) => {
                let rest = tl_merge(m_node.left, m_node.right, self.op, &self.e);
                self.root =
                    tl_merge(tl_merge(l, rest, self.op, &self.e), r, self.op, &self.e);
                true
            }
        }
    }

    /// Remove all occurrences of val. Returns the number of removed elements.
    pub fn erase_all(&mut self, val: V) -> usize {
        let root = self.root.take();
        let (l, mr) = tl_split(root, &val, self.op, &self.e);
        let (m, r) = tl_split_upper(mr, &val, self.op, &self.e);
        let removed = tl_cnt(&m);
        self.root = tl_merge(l, r, self.op, &self.e);
        removed
    }

    pub fn contains(&self, val: V) -> bool {
        let mut cur = &self.root;
        while let Some(n) = cur {
            match val.cmp(&n.val) {
                Ordering::Less => cur = &n.left,
                Ordering::Equal => return true,
                Ordering::Greater => cur = &n.right,
            }
        }
        false
    }

    /// Number of occurrences of val
    pub fn count(&self, val: V) -> usize {
        self.count_le(val.clone()) - self.count_lt(val)
    }

    /// k-th smallest value (0-indexed)
    pub fn kth(&self, k: usize) -> Option<&V> {
        tl_kth(&self.root, k)
    }

    /// Number of elements strictly less than val
    pub fn count_lt(&self, val: V) -> usize {
        let mut cur = &self.root;
        let mut cnt = 0;
        while let Some(n) = cur {
            if n.val < val {
                cnt += 1 + tl_cnt(&n.left);
                cur = &n.right;
            } else {
                cur = &n.left;
            }
        }
        cnt
    }

    /// Number of elements <= val
    pub fn count_le(&self, val: V) -> usize {
        let mut cur = &self.root;
        let mut cnt = 0;
        while let Some(n) = cur {
            if n.val <= val {
                cnt += 1 + tl_cnt(&n.left);
                cur = &n.right;
            } else {
                cur = &n.left;
            }
        }
        cnt
    }

    /// Smallest value in the treap
    pub fn min(&self) -> Option<&V> {
        let mut cur = &self.root;
        let mut res: Option<&V> = None;
        while let Some(n) = cur {
            res = Some(&n.val);
            cur = &n.left;
        }
        res
    }

    /// Largest value in the treap
    pub fn max(&self) -> Option<&V> {
        let mut cur = &self.root;
        let mut res: Option<&V> = None;
        while let Some(n) = cur {
            res = Some(&n.val);
            cur = &n.right;
        }
        res
    }

    /// Smallest value >= val
    pub fn lower_bound(&self, val: V) -> Option<&V> {
        let mut cur = &self.root;
        let mut res: Option<&V> = None;
        while let Some(n) = cur {
            if n.val >= val {
                res = Some(&n.val);
                cur = &n.left;
            } else {
                cur = &n.right;
            }
        }
        res
    }

    /// Smallest value > val
    pub fn upper_bound(&self, val: V) -> Option<&V> {
        let mut cur = &self.root;
        let mut res: Option<&V> = None;
        while let Some(n) = cur {
            if n.val > val {
                res = Some(&n.val);
                cur = &n.left;
            } else {
                cur = &n.right;
            }
        }
        res
    }

    /// Aggregate values in sorted order for indices [l, r) (0-indexed)
    pub fn prod(&mut self, l: usize, r: usize) -> V {
        let root = self.root.take();
        let (left, mr) = tl_split_at(root, l, self.op, &self.e);
        let (mid, right) = tl_split_at(mr, r - l, self.op, &self.e);
        let res = mid.as_ref().map_or(self.e.clone(), |m| m.acc.clone());
        self.root = tl_merge(
            tl_merge(left, mid, self.op, &self.e),
            right,
            self.op,
            &self.e,
        );
        res
    }

    /// Starting from index l, find the smallest r such that
    /// f(prod(l, r)) is true but f(prod(l, r+1)) is false.
    /// Returns r (the index where f first fails).
    /// Precondition: f(e) is true.
    pub fn max_right(&mut self, l: usize, f: impl Fn(&V) -> bool) -> usize {
        let root = self.root.take();
        let (left, right) = tl_split_at(root, l, self.op, &self.e);
        let (_, cnt) = tl_max_right(&right, self.e.clone(), self.op, &self.e, &f);
        self.root = tl_merge(left, right, self.op, &self.e);
        l + cnt
    }

    /// Starting from index r, going left, find the largest l such that
    /// f(prod(l, r)) is true but f(prod(l-1, r)) is false.
    /// Returns l (the index where f first fails going left).
    /// Precondition: f(e) is true.
    pub fn min_left(&mut self, r: usize, f: impl Fn(&V) -> bool) -> usize {
        let root = self.root.take();
        let (left, right) = tl_split_at(root, r, self.op, &self.e);
        let (_, cnt) = tl_min_left(&left, self.e.clone(), self.op, &self.e, &f);
        self.root = tl_merge(left, right, self.op, &self.e);
        r - cnt
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
