use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }
    let mut m: usize = 0;
    for s in &ss {
        m = max(m, s.len());
    }
    let mut ans: Vec<String> = Vec::new();
    for s in &ss {
        let k: usize = (m - s.len()) / 2;
        let prefix = ".".repeat(k);
        let suffix = ".".repeat(k);
        ans.push(prefix + &s + &suffix);
    }
    for s in ans {
        println!("{}", s);
    }
}
