mod common;
use common::*;

#[test]
fn empty_treap() {
    let mut t = sum_treap();
    assert_eq!(t.prod(0, 100), 0);
    assert_eq!(t.max_right(0, |x| *x < 10), None);
    assert_eq!(t.min_left(100, |x| *x < 10), None);
    assert_eq!(t.len(), 0);
    assert_eq!(t.key_min(), None);
    assert_eq!(t.key_max(), None);
}

#[test]
fn single_element() {
    let mut t = sum_treap();
    t.insert(5, 42);
    assert_eq!(t.prod(5, 6), 42);
    assert_eq!(t.prod(0, 5), 0);
    assert_eq!(t.prod(6, 10), 0);
    assert_eq!(t.max_right(5, |x| *x < 50), None);
    assert_eq!(t.max_right(5, |x| *x < 10), Some(5));
    assert_eq!(t.min_left(6, |x| *x < 50), None);
    assert_eq!(t.min_left(6, |x| *x < 10), Some(5));
}

#[test]
fn insert_overwrite_no_duplicate() {
    let mut t = sum_treap();
    t.insert(3, 10);
    t.insert(3, 20);
    t.insert(3, 30);
    assert_eq!(t.len(), 1);
    assert_eq!(t.prod(3, 4), 30);
}

#[test]
fn contains_and_erase() {
    let mut t = sum_treap();
    t.insert(1, 10);
    t.insert(3, 20);
    t.insert(5, 30);
    assert!(t.contains(3));
    assert!(!t.contains(2));
    assert!(t.erase(3));
    assert!(!t.contains(3));
    assert!(!t.erase(3));
    assert_eq!(t.len(), 2);
}

#[test]
fn kth() {
    let mut t = sum_treap();
    t.insert(5, 50);
    t.insert(1, 10);
    t.insert(3, 30);
    t.insert(7, 70);
    assert_eq!(t.kth(0), Some((&1, &10)));
    assert_eq!(t.kth(1), Some((&3, &30)));
    assert_eq!(t.kth(2), Some((&5, &50)));
    assert_eq!(t.kth(3), Some((&7, &70)));
    assert_eq!(t.kth(4), None);
}

#[test]
fn count_lt_le() {
    let mut t = sum_treap();
    t.insert(1, 0);
    t.insert(3, 0);
    t.insert(5, 0);
    t.insert(7, 0);
    assert_eq!(t.count_lt(0), 0);
    assert_eq!(t.count_lt(1), 0);
    assert_eq!(t.count_lt(3), 1);
    assert_eq!(t.count_lt(4), 2);
    assert_eq!(t.count_lt(8), 4);
    assert_eq!(t.count_le(0), 0);
    assert_eq!(t.count_le(1), 1);
    assert_eq!(t.count_le(3), 2);
    assert_eq!(t.count_le(7), 4);
    assert_eq!(t.count_le(8), 4);
}

#[test]
fn lower_upper_bound() {
    let mut t = sum_treap();
    t.insert(1, 0);
    t.insert(3, 0);
    t.insert(5, 0);
    t.insert(7, 0);
    assert_eq!(t.lower_bound(0), Some(&1));
    assert_eq!(t.lower_bound(1), Some(&1));
    assert_eq!(t.lower_bound(2), Some(&3));
    assert_eq!(t.lower_bound(7), Some(&7));
    assert_eq!(t.lower_bound(8), None);
    assert_eq!(t.upper_bound(0), Some(&1));
    assert_eq!(t.upper_bound(1), Some(&3));
    assert_eq!(t.upper_bound(5), Some(&7));
    assert_eq!(t.upper_bound(7), None);
}

#[test]
fn get_existing_and_missing() {
    let mut t = sum_treap();
    t.insert(1, 10);
    t.insert(3, 30);
    t.insert(5, 50);
    assert_eq!(t.get(1), Some(&10));
    assert_eq!(t.get(3), Some(&30));
    assert_eq!(t.get(5), Some(&50));
    assert_eq!(t.get(0), None);
    assert_eq!(t.get(2), None);
    assert_eq!(t.get(6), None);
}

#[test]
fn get_empty_treap() {
    let t = sum_treap();
    assert_eq!(t.get(0), None);
}

#[test]
fn update_existing_key() {
    let mut t = sum_treap();
    t.insert(1, 10);
    t.insert(3, 30);
    t.insert(5, 50);
    assert!(t.update(3, 99));
    assert_eq!(t.get(3), Some(&99));
    assert_eq!(t.prod(1, 6), 10 + 99 + 50);
    assert_eq!(t.len(), 3);
}

#[test]
fn update_missing_key() {
    let mut t = sum_treap();
    t.insert(1, 10);
    assert!(!t.update(2, 20));
    assert_eq!(t.get(2), None);
    assert_eq!(t.len(), 1);
}

#[test]
fn update_preserves_prod() {
    let mut t = sum_treap();
    for i in 0..10 {
        t.insert(i, i as i64);
    }
    // sum of [0, 10) = 0+1+...+9 = 45
    assert_eq!(t.prod(0, 10), 45);
    t.update(5, 100);
    // 45 - 5 + 100 = 140
    assert_eq!(t.prod(0, 10), 140);
    assert_eq!(t.prod(5, 6), 100);
    assert_eq!(t.prod(0, 5), 10); // 0+1+2+3+4
}

#[test]
fn key_min_max() {
    let mut t = sum_treap();
    assert_eq!(t.key_min(), None);
    assert_eq!(t.key_max(), None);

    t.insert(5, 50);
    assert_eq!(t.key_min(), Some(&5));
    assert_eq!(t.key_max(), Some(&5));

    t.insert(1, 10);
    t.insert(9, 90);
    t.insert(3, 30);
    assert_eq!(t.key_min(), Some(&1));
    assert_eq!(t.key_max(), Some(&9));

    t.erase(1);
    assert_eq!(t.key_min(), Some(&3));
    assert_eq!(t.key_max(), Some(&9));

    t.erase(9);
    assert_eq!(t.key_min(), Some(&3));
    assert_eq!(t.key_max(), Some(&5));
}
