mod common;
use common::*;

#[test]
fn sum() {
    let mut t = sum_treap();
    t.insert(1, 10);
    t.insert(3, 20);
    t.insert(5, 30);
    t.insert(7, 40);

    // From key 1, accumulate: 10, 30, 60, 100
    assert_eq!(t.max_right(1, |x| *x < 35), Some(5));
    assert_eq!(t.max_right(1, |x| *x < 15), Some(3));
    assert_eq!(t.max_right(1, |x| *x < 5), Some(1));
    assert_eq!(t.max_right(1, |x| *x < 200), None);

    // From key 5: 30, 70
    assert_eq!(t.max_right(5, |x| *x < 50), Some(7));
    assert_eq!(t.max_right(5, |x| *x < 25), Some(5));

    // No keys >= 100
    assert_eq!(t.max_right(100, |x| *x < 1), None);
}

#[test]
fn min() {
    let mut t = min_treap();
    t.insert(1, 50);
    t.insert(3, 10);
    t.insert(5, 30);
    t.insert(7, 20);

    // From key 1, running min: 50, 10, 10, 10
    assert_eq!(t.max_right(1, |x| *x > 15), Some(3));
    assert_eq!(t.max_right(1, |x| *x > 5), None);
}

#[test]
fn max() {
    let mut t = max_treap();
    t.insert(1, 10);
    t.insert(3, 20);
    t.insert(5, 5);
    t.insert(7, 50);

    // From key 1, running max: 10, 20, 20, 50
    assert_eq!(t.max_right(1, |x| *x < 25), Some(7));
    assert_eq!(t.max_right(1, |x| *x < 15), Some(3));
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
        for start in 0..=12 {
            let mut acc = 0i64;
            let mut expected = None;
            for &(k, v) in &sorted_data {
                if k < start {
                    continue;
                }
                acc += v;
                if acc >= threshold {
                    expected = Some(k);
                    break;
                }
            }
            let result = t.max_right(start, |x| *x < threshold);
            assert_eq!(
                result, expected,
                "max_right({}, |x| x < {})",
                start, threshold
            );
        }
    }
}
