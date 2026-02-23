mod common;
use common::*;

#[test]
fn sum() {
    let mut t = sum_treap();
    t.insert(1, 10);
    t.insert(3, 20);
    t.insert(5, 30);
    t.insert(7, 40);

    // min_left(8): keys < 8 are {1,3,5,7}, from right: 40, 70, 90, 100
    assert_eq!(t.min_left(8, |x| *x < 55), Some(5));
    assert_eq!(t.min_left(8, |x| *x < 35), Some(7));
    assert_eq!(t.min_left(8, |x| *x < 200), None);

    // min_left(6): keys < 6 are {1,3,5}, from right: 30, 50, 60
    assert_eq!(t.min_left(6, |x| *x < 45), Some(3));
    assert_eq!(t.min_left(6, |x| *x < 25), Some(5));

    // No keys < 0
    assert_eq!(t.min_left(0, |x| *x < 1), None);
}

#[test]
fn min() {
    let mut t = min_treap();
    t.insert(1, 50);
    t.insert(3, 10);
    t.insert(5, 30);
    t.insert(7, 20);

    // From right, running min: 20, 20, 10, 10
    assert_eq!(t.min_left(8, |x| *x > 15), Some(3));
    assert_eq!(t.min_left(8, |x| *x > 5), None);
}

#[test]
fn max() {
    let mut t = max_treap();
    t.insert(1, 10);
    t.insert(3, 20);
    t.insert(5, 5);
    t.insert(7, 50);

    // From right, running max: 50, 50, 50, 50
    assert_eq!(t.min_left(8, |x| *x < 25), Some(7));

    // keys < 7: {1,3,5}, from right: 5, 20, 20
    assert_eq!(t.min_left(7, |x| *x < 15), Some(3));
}

#[test]
fn brute_force() {
    let mut t = sum_treap();
    let data = test_data();
    for &(k, v) in &data {
        t.insert(k, v);
    }
    let mut sorted_data = data.clone();
    sorted_data.sort();

    for threshold in [5, 10, 15, 20, 30, 50, 100] {
        for end in 0..=12 {
            let mut acc = 0i64;
            let mut expected = None;
            for &(k, v) in sorted_data.iter().rev() {
                if k >= end {
                    continue;
                }
                acc += v;
                if acc >= threshold {
                    expected = Some(k);
                    break;
                }
            }
            let result = t.min_left(end, |x| *x < threshold);
            assert_eq!(
                result, expected,
                "min_left({}, |x| x < {})",
                end, threshold
            );
        }
    }
}
