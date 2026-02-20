use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    const M: usize = 1000000;
    let mut b = vec![0; M];
    for i in 0..n {
        b[0] += 1;
        b[a[i]] -= 1;
    }
    for i in 1..M {
        b[i] += b[i - 1];
    }
    for i in 0..(M - 1) {
        b[i + 1] += b[i] / 10;
        b[i] %= 10;
    }
    while b.len() > 1 && b.last() == Some(&0) {
        b.pop();
    }
    println!(
        "{}",
        b.iter()
            .rev()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    )
}
