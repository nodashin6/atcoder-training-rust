#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        s: usize,
        t: usize,
        uvc: [(usize, usize, usize); m],
    }
    let mut graph = vec![vec![]; n];
    for (u, v, c) in uvc {
        graph[u - 1].push((v - 1, c));
    }
    let mut queue = std::collections::VecDeque::new();
    // (index, cost, depth)
    queue.push_back((0, 0, 0));
    let mut ans = vec![false; n];
    while let Some((index, cost, depth)) = queue.pop_front() {
        if depth == l {
            if s <= cost && cost <= t {
                ans[index] = true;
            }
        } else if depth < l {
            for (next_index, next_cost) in &graph[index] {
                queue.push_back((*next_index, cost + next_cost, depth + 1));
            }
        }
    }
    println!(
        "{}",
        ans.into_iter()
            .enumerate()
            .filter(|(_, is_ok)| *is_ok)
            .map(|(i, _)| i + 1)
            .join(" ")
    )
}
