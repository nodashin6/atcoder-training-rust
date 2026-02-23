use ac_tree::treap::Treap;

#[allow(dead_code)]
pub fn sum_treap() -> Treap<usize, i64> {
    Treap::new(|a, b| a + b, 0)
}

#[allow(dead_code)]
pub fn min_treap() -> Treap<usize, i64> {
    Treap::new(|a, b| *a.min(b), i64::MAX)
}

#[allow(dead_code)]
pub fn max_treap() -> Treap<usize, i64> {
    Treap::new(|a, b| *a.max(b), i64::MIN)
}

#[allow(dead_code)]
pub fn test_data() -> Vec<(usize, i64)> {
    vec![
        (2, 5), (8, 3), (1, 7), (5, 11), (3, 2),
        (10, 6), (4, 9), (7, 1), (6, 4), (9, 8),
    ]
}
