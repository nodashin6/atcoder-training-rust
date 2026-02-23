mod common;
use common::*;

// ============================================================
// prod: range sum
// ============================================================
#[test]
fn sum_basic() {
    let mut t = sum_treap();
    t.insert(1, 10);
    t.insert(3, 20);
    t.insert(5, 30);
    t.insert(7, 40);

    assert_eq!(t.prod(1, 8), 100);
    assert_eq!(t.prod(1, 5), 30);
    assert_eq!(t.prod(3, 6), 50);
    assert_eq!(t.prod(5, 8), 70);
    assert_eq!(t.prod(3, 4), 20);
    assert_eq!(t.prod(2, 2), 0);
    assert_eq!(t.prod(2, 3), 0);
    assert_eq!(t.prod(0, 100), 100);
}

#[test]
fn sum_after_insert_overwrite() {
    let mut t = sum_treap();
    t.insert(5, 100);
    assert_eq!(t.prod(0, 10), 100);
    t.insert(5, 200);
    assert_eq!(t.prod(0, 10), 200);
    assert_eq!(t.len(), 1);
}

#[test]
fn sum_after_erase() {
    let mut t = sum_treap();
    t.insert(1, 10);
    t.insert(3, 20);
    t.insert(5, 30);
    assert_eq!(t.prod(1, 6), 60);
    t.erase(3);
    assert_eq!(t.prod(1, 6), 40);
    assert_eq!(t.len(), 2);
}

#[test]
fn sum_large_keys() {
    let mut t = sum_treap();
    t.insert(1_000_000_000, 10);
    t.insert(2_000_000_000, 20);
    t.insert(3_000_000_000, 30);
    assert_eq!(t.prod(0, 4_000_000_000), 60);
    assert_eq!(t.prod(1_500_000_000, 2_500_000_000), 20);
}

// ============================================================
// prod: range min
// ============================================================
#[test]
fn min_basic() {
    let mut t = min_treap();
    t.insert(1, 50);
    t.insert(3, 10);
    t.insert(5, 30);
    t.insert(7, 20);

    assert_eq!(t.prod(1, 8), 10);
    assert_eq!(t.prod(1, 3), 50);
    assert_eq!(t.prod(1, 4), 10);
    assert_eq!(t.prod(5, 8), 20);
    assert_eq!(t.prod(4, 6), 30);
    assert_eq!(t.prod(2, 3), i64::MAX);
}

#[test]
fn min_single() {
    let mut t = min_treap();
    t.insert(42, -100);
    assert_eq!(t.prod(0, 100), -100);
    assert_eq!(t.prod(43, 100), i64::MAX);
}

// ============================================================
// prod: range max
// ============================================================
#[test]
fn max_basic() {
    let mut t = max_treap();
    t.insert(1, 50);
    t.insert(3, 10);
    t.insert(5, 30);
    t.insert(7, 20);

    assert_eq!(t.prod(1, 8), 50);
    assert_eq!(t.prod(3, 8), 30);
    assert_eq!(t.prod(5, 8), 30);
    assert_eq!(t.prod(7, 8), 20);
    assert_eq!(t.prod(2, 3), i64::MIN);
}

// ============================================================
// prod: brute-force verification
// ============================================================
#[test]
fn sum_brute_force() {
    let mut t = sum_treap();
    let data = test_data();
    for &(k, v) in &data {
        t.insert(k, v);
    }
    for l in 0..=12 {
        for r in l..=12 {
            let expected: i64 = data
                .iter()
                .filter(|&&(k, _)| k >= l && k < r)
                .map(|&(_, v)| v)
                .sum();
            assert_eq!(t.prod(l, r), expected, "sum prod({}, {})", l, r);
        }
    }
}

#[test]
fn min_brute_force() {
    let mut t = min_treap();
    let data = test_data();
    for &(k, v) in &data {
        t.insert(k, v);
    }
    for l in 0..=12 {
        for r in l..=12 {
            let expected = data
                .iter()
                .filter(|&&(k, _)| k >= l && k < r)
                .map(|&(_, v)| v)
                .min()
                .unwrap_or(i64::MAX);
            assert_eq!(t.prod(l, r), expected, "min prod({}, {})", l, r);
        }
    }
}

#[test]
fn max_brute_force() {
    let mut t = max_treap();
    let data = test_data();
    for &(k, v) in &data {
        t.insert(k, v);
    }
    for l in 0..=12 {
        for r in l..=12 {
            let expected = data
                .iter()
                .filter(|&&(k, _)| k >= l && k < r)
                .map(|&(_, v)| v)
                .max()
                .unwrap_or(i64::MIN);
            assert_eq!(t.prod(l, r), expected, "max prod({}, {})", l, r);
        }
    }
}
