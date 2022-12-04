use itertools::Itertools;
use regex::Regex;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    let overlapping_entirely = buffer
        .split_terminator("\n")
        .map(|assignments| re.captures_iter(assignments))
        .map(|mut captures| {
            let (a, b, c, d) = captures
                .next()
                .unwrap()
                .iter()
                .skip(1)
                .take(4)
                .map(|m| m.unwrap().as_str().parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap();
            return (a, b, c, d);
        })
        .filter(|(a, b, c, d)| !((a < c && b < c) | (a > d && b > d)))
        .count();

    println!("{overlapping_entirely}");
}
