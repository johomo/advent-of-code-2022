use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

fn is_marker(substr: &str) -> bool {
    let set: HashSet<char> = HashSet::from_iter(substr.chars());
    set.len() == substr.len()
}

fn main() {
    let marker_size: usize = 14;
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let marker_index = (marker_size..buffer.len())
        .map_while(|index| {
            let substr = &buffer[index - marker_size..index];
            match is_marker(substr) {
                false => Some(index),
                true => None,
            }
        })
        .last()
        .unwrap()
        + 1;

    println!("{}", marker_index);
}
