use proconio::input;

fn main() {
    input! {
        q: usize,
        aa: [usize; q],
    }

    let mut volume = 0;
    const VOLUME_REQUIRE: usize = 3;
    let mut is_play = false;
    for a in aa {
        match a {
            1 => {
                volume += 1;
            }
            2 => {
                if volume > 0 {
                    volume -= 1;
                }
            }
            3 => {
                is_play = !is_play;
            }
            _ => {}
        }
        let ans = is_play && volume >= VOLUME_REQUIRE;
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
