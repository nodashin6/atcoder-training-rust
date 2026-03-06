use ac_tree::binary_indexed_tree::BinaryIndexedTree;

#[test]
fn empty() {
    let bit = BinaryIndexedTree::new(10);
    for i in 0..10 {
        assert_eq!(bit.sum(i), 0);
    }
    assert_eq!(bit.range_sum(0, 10), 0);
}

#[test]
fn single_add() {
    let mut bit = BinaryIndexedTree::new(10);
    bit.add(5, 42);
    assert_eq!(bit.sum(4), 0);
    assert_eq!(bit.sum(5), 42);
    assert_eq!(bit.sum(9), 42);
}

#[test]
fn multiple_adds_same_index() {
    let mut bit = BinaryIndexedTree::new(10);
    bit.add(3, 10);
    bit.add(3, 20);
    bit.add(3, -5);
    assert_eq!(bit.sum(3), 25);
}

#[test]
fn prefix_sum() {
    let mut bit = BinaryIndexedTree::new(5);
    for i in 0..5 {
        bit.add(i, (i + 1) as i64);
    }
    // values: [1, 2, 3, 4, 5]
    assert_eq!(bit.sum(0), 1);
    assert_eq!(bit.sum(1), 3);
    assert_eq!(bit.sum(2), 6);
    assert_eq!(bit.sum(3), 10);
    assert_eq!(bit.sum(4), 15);
}

#[test]
fn range_sum_basic() {
    let mut bit = BinaryIndexedTree::new(5);
    for i in 0..5 {
        bit.add(i, (i + 1) as i64);
    }
    assert_eq!(bit.range_sum(0, 5), 15);
    assert_eq!(bit.range_sum(1, 4), 9); // 2+3+4
    assert_eq!(bit.range_sum(2, 3), 3);
    assert_eq!(bit.range_sum(0, 1), 1);
    assert_eq!(bit.range_sum(4, 5), 5);
}

#[test]
fn range_sum_empty() {
    let mut bit = BinaryIndexedTree::new(5);
    bit.add(2, 100);
    assert_eq!(bit.range_sum(3, 3), 0);
    assert_eq!(bit.range_sum(5, 3), 0);
}

#[test]
fn negative_values() {
    let mut bit = BinaryIndexedTree::new(5);
    bit.add(0, -10);
    bit.add(1, 20);
    bit.add(2, -30);
    assert_eq!(bit.sum(0), -10);
    assert_eq!(bit.sum(1), 10);
    assert_eq!(bit.sum(2), -20);
    assert_eq!(bit.range_sum(0, 3), -20);
    assert_eq!(bit.range_sum(1, 3), -10);
}

#[test]
fn brute_force() {
    let n = 100;
    let mut bit = BinaryIndexedTree::new(n);
    let mut arr = vec![0i64; n];
    for i in 0..n {
        let v = (i as i64 * 7 + 3) % 19 - 9;
        bit.add(i, v);
        arr[i] = v;
    }
    for l in 0..n {
        let mut expected = 0i64;
        for r in l..n {
            expected += arr[r];
            assert_eq!(
                bit.range_sum(l, r + 1),
                expected,
                "range_sum({}, {})",
                l,
                r + 1
            );
        }
    }
}
