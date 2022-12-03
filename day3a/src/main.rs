use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).unwrap();

    let priorities: u32 = buffer
        .split_terminator("\n")
        // Split rucksack items by compartments
        .map(|rucksack| {
            // str::len works fine with ASCII
            let len = rucksack.len();
            (&rucksack[0..len / 2], &rucksack[len / 2..len])
        })
        // Get item appearing both in first and second compartments
        .map(|(first, second)| {
            let first_items: HashSet<char> = HashSet::from_iter(first.chars());
            let second_items: HashSet<char> = HashSet::from_iter(second.chars());
            first_items
                .intersection(&second_items)
                .next()
                .unwrap()
                .clone()
        })
        // Get priority of this item
        .map(|item| match item.is_ascii_uppercase() {
            true => item as u32 - 64 + 26,
            false => item as u32 - 96,
        })
        .sum();

    println!("{priorities}")
}
