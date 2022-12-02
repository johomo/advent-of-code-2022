use std::collections::BinaryHeap;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut heap = BinaryHeap::from_iter(buffer.split("\n\n").map(|self_calories| {
        self_calories
            .split("\n")
            .map(|calories| calories.parse::<u64>().unwrap_or(0))
            .sum()
    }));

    // BinaryHeap::into_iter_sorted requires nightly builds.
    // let top_three_sum = heap.into_iter_sorted().take(3).sum();

    // Ugly, I know...
    let top_three_sum = heap.pop().unwrap_or(0) + heap.pop().unwrap_or(0) + heap.pop().unwrap_or(0);

    println!("{top_three_sum}")
}
