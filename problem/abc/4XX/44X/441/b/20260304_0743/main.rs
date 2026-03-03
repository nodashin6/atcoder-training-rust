#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn is_lang(ss: &[char], ww: &[char]) -> bool {
    for w in ww {
        if !ss.contains(&w) {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        _n: usize,
        _m: usize,
        ss: Chars,
        tt: Chars,
        q: usize,
        www: [Chars; q],
    }
    for ww in &www {
        let is_takahashi = is_lang(&ss, &ww);
        let is_aoki = is_lang(&tt, &ww);
        if is_takahashi && is_aoki {
            println!("Unknown");
        } else if is_takahashi {
            println!("Takahashi");
        } else if is_aoki {
            println!("Aoki");
        } else {
            println!("Unknown");
        }
    }
}
