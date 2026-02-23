use proconio::input;

const REOPEN: usize = 100;

fn main() {
    input! {
        n: usize,
        t: usize,
        mut aa: [usize; n],
    }
    aa.push(t);
    let mut open = 0;
    let mut ans = 0;
    for i in 0..(n+1) {
        let ai = aa[i];
        if open <= ai {
            ans += ai - open;
            open = ai + REOPEN;
        }
    }
    println!("{}", ans);
}
