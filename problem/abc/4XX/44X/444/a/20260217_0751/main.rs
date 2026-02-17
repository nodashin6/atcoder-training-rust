use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let ans = (s[0] == s[1]) && (s[1] == s[2]);
    println!("{}", if ans { "Yes" } else { "No" });
}
