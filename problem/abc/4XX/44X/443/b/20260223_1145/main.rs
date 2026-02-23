use proconio::input;

fn main() {
    input! {
        mut n: usize,
        k: usize,
    }
    let mut eaten = 0;
    for i in 0..k {
        eaten += n;
        if eaten >= k {
            println!("{}", i);
            break
        }
        n += 1;
    }
}
