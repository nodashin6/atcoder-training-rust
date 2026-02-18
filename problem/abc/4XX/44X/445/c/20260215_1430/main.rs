use proconio::input;

fn simulate(a: &[usize], ans: &[usize], mut i: usize) -> usize {
    while a[i] != i {
        let j = a[i];
        if 0 < ans[j] {
            return ans[j];
        }
        i = j;
    }
    return i;
}

fn main() {
    input! {
        n: usize,
        a_input: [usize; n],
    }
    // 0-indexed
    let a: Vec<usize> = a_input.iter().map(|&x| x - 1).collect();
    let mut ans = vec![0; n];
    for i in (0..n).rev() {
        ans[i] = simulate(&a, &ans, i);
    }
    println!(
        "{}",
        ans.iter()
            .map(|&x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}
