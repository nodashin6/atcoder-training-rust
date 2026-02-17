use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut ans = 0;
    for mut i in 1..(n + 1) {
        let mut order_sum = 0;
        while i > 0 {
            order_sum += i % 10;
            i /= 10;
        }
        if order_sum == k {
            ans += 1;
        }
    }
    println!("{}", ans);
}
