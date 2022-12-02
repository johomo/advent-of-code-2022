use std::io;
use std::io::prelude::*;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).unwrap();

    let max_calories: u64 = buffer
        .split("\n\n")
        .map(|self_calories| {
            self_calories
                .split("\n")
                .map(|calories| calories.parse::<u64>().unwrap_or(0))
                .sum()
        })
        .max()
        .unwrap();

    println!("{max_calories}")
}
