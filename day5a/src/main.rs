use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;
use std::str::Chars;

type Deque = VecDeque<char>;

#[derive(Debug)]
struct Crate {
    supply: Option<char>,
}

struct CratesIterator<'a> {
    row: &'a mut Chars<'a>,
}

impl<'a> Iterator for CratesIterator<'a> {
    type Item = Crate;

    fn next(&mut self) -> Option<Self::Item> {
        match self.row.take(4).collect::<Vec<char>>().iter().nth(1) {
            Some(supply) => match supply {
                ' ' => Some(Crate { supply: None }),
                _ => Some(Crate {
                    supply: Some(supply.clone()),
                }),
            },
            None => None,
        }
    }
}

#[derive(Debug)]
struct Command {
    amount: usize,
    from: usize,
    to: usize,
}

impl Command {
    fn from_str(command: &str) -> Command {
        let chunks = command.split_terminator(" ").collect::<Vec<&str>>();
        Command {
            amount: chunks[1].parse::<usize>().unwrap(),
            from: chunks[3].parse::<usize>().unwrap(),
            to: chunks[5].parse::<usize>().unwrap(),
        }
    }
}

fn parse_stacks(buffer: &str) -> Vec<Deque> {
    let mut stacks = Vec::<Deque>::new();

    buffer
        .split_terminator("\n")
        .map(|row| {
            CratesIterator {
                row: &mut row.chars(),
            }
            .collect::<Vec<Crate>>()
        })
        .map(|crates_row| {
            if stacks.len() == 0 {
                stacks.resize_with(crates_row.len(), Deque::new);
            }
            crates_row
                .iter()
                .enumerate()
                .map(|(idx, crate_)| {
                    if let Some(supply) = crate_.supply {
                        stacks[idx].push_front(supply.clone());
                    }
                })
                .last();
        })
        .last();
    return stacks;
}

fn do_commands(stacks: &mut Vec<Deque>, commands: &str) -> () {
    commands
        .split_terminator("\n")
        .map(|command| Command::from_str(command))
        .map(|command| {
            std::iter::repeat(())
                .take(command.amount)
                .map(|()| {
                    let supply = stacks[command.from - 1].pop_back().unwrap();
                    stacks[command.to - 1].push_back(supply);
                })
                .last();
        })
        .last();
}

fn top_crates(stacks: &mut Vec<Deque>) -> String {
    stacks
        .iter()
        .map(|stack| stack.back())
        .flatten()
        .collect::<String>()
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let split_buffer: Vec<&str> = buffer.split_terminator("\n\n").collect();
    let crates_buffer = split_buffer[0];
    let commands_buffer = split_buffer[1];

    let mut stacks = parse_stacks(crates_buffer);
    do_commands(&mut stacks, commands_buffer);

    println!("{}", top_crates(&mut stacks));
}
