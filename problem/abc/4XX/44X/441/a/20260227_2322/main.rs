#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input;

fn is_in_range(p: usize, q: usize, x: usize, y: usize) -> bool {
    if p <= x && x < (p + 100) {
        if q <= y && y < (q + 100) {
            return true;
        }
    }
    return false;
}

fn main() {
    input! {
        p: usize,
        q: usize,
        x: usize,
        y: usize,
    }
    if is_in_range(p, q, x, y) {
        println!("Yes");
    } else {
        println!("No");
    }
}
