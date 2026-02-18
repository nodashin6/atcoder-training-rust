use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let chars: Vec<char> = s.chars().collect();
    let n: usize = chars.len();
    let ans = chars[0] == chars[n - 1];
    println!("{}", if ans { "Yes" } else { "No" });
}
