use std::io;
use std::io::prelude::*;

fn score(opponent: &str, me: &str) -> u64 {
    // Oponent plays A (Rock), B (Paper), C (Scissors)
    //       I play  X (Rock), Y (Paper), Z (Scissors)
    let score_by_choosing = match me {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };

    let score_by_playing: u64 = match (opponent, me) {
        ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
        _ => 0,
    };

    return score_by_choosing + score_by_playing;
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).unwrap();

    let score: u64 = buffer
        .split("\n")
        .map(|round| round.split_whitespace().collect::<Vec<_>>())
        .map(|round| score(round[0], round[1]))
        .sum();

    println!("{score}");
}
