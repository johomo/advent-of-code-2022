use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

// Iterator of groups of Elves from a list of rucksacks (input of this problem)
struct GroupIterator<'a> {
    // Number of elves in each group
    size: usize,
    // Iterator of ruckacks.
    iterator: &'a mut dyn Iterator<Item = &'a str>,
}

impl<'a> Iterator for GroupIterator<'a> {
    type Item = Vec<&'a str>;

    // Returns a vector with all rucksacks in a group
    fn next(&mut self) -> Option<Self::Item> {
        let group: Vec<&'a str> = self.iterator.take(self.size).collect();
        match group.len() {
            0 => None,
            _ => Some(group),
        }
    }
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).unwrap();

    let elves_iterator: &mut dyn Iterator<Item = &str> = &mut buffer.split_terminator("\n");

    let group_iterator = GroupIterator {
        size: 3,
        iterator: elves_iterator,
    };

    // Iterate over groups of elves. Get score of each group
    let priorities: u32 = group_iterator
        .map(|group| {
            // Iterate over all elfs rucksacks in the group.
            group
                .into_iter()
                .map(|rucksack| HashSet::<char>::from_iter(rucksack.chars()))
                // Get common type of all elves within a group.
                .reduce(|first, second| {
                    // Use of copied is inefficient
                    first.intersection(&second).copied().collect()
                })
                // Extract the common item from the HashSet
                .map(|common_items| common_items.iter().next().unwrap().clone())
                // Get priority of this item
                .map(|item| match item.is_ascii_uppercase() {
                    true => item as u32 - 64 + 26,
                    false => item as u32 - 96,
                })
                .unwrap()
        })
        .sum();

    println!("{priorities}")
}
