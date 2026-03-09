#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::collections::BinaryHeap;

fn collect_top_k(heap: &mut BinaryHeap<(i64, Vec<i64>)>, k: usize) {
    // 先頭 K 個だけ取り出して、新たなヒープを作成する
    let mut new_heap = BinaryHeap::new();
    for _ in 0..k {
        if let Some((score, nums)) = heap.pop() {
            new_heap.push((score, nums));
        } else {
            break;
        }
    }
    *heap = new_heap;
}

fn main() {
    input! {
        n: usize,
        k: i64,
        x: usize,
        mut aa: [i64; n],
    }
    aa.sort();
    aa.reverse();

    let mut heap = BinaryHeap::new();
    let score = aa[0] * k;
    let mut nums = vec![0; n];
    nums[0] = k;
    let node = (score, nums);
    heap.push(node);
    for _ in 0..x {
        let (score, nums) = heap.pop().unwrap();
        println!("{}", score);
        // println!("{:?}", nums);

        let mut most_right_index: usize = 1;
        for i in most_right_index..n {
            if nums[i] > 0 {
                most_right_index = i;
            }
        }
        if nums[0] == 0 {
            continue;
        };
        for i in most_right_index..n {
            // 0 番目を 1 減らして i 番目を 1 増やす
            let new_score = score + aa[i] - aa[0];
            let mut new_nums = nums.clone();
            new_nums[i] += 1;
            new_nums[0] -= 1;
            heap.push((new_score, new_nums));
        }

        // メモリが爆発しないように、ヒープのサイズを制限する
        if heap.len() > (10 * x) {
            collect_top_k(&mut heap, x);
        }
    }
}
