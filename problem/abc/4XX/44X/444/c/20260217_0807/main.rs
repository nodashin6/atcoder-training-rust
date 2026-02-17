use proconio::input;

fn is_ok(a: &[usize], target: &usize) -> bool {
    let mut b: Vec<_> = a.iter().filter(|&x| x < target).collect();
    if b.len() % 2 > 0 {
        return false;
    }
    b.sort();
    for i in 0..(b.len() / 2) {
        let j = b.len() - 1 - i;
        if (b[i] + b[j]) != *target {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let s: usize = a.iter().sum();
    let max_a = *a.iter().max().unwrap();
    let mut ans = vec![];
    for i in 1..(n + 1) {
        if s % i > 0 {
            continue;
        }
        let target = s / i;
        if target < max_a {
            continue;
        }
        if is_ok(&a, &target) {
            ans.push(target);
        }
    }
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" "),
    )
}
