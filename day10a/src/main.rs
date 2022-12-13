use std::io;
use std::io::prelude::*;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut strength = 0;
    let mut register: i32 = 1;
    let mut cycle: i32 = 1;
    let strength_cycles: [i32; 6] = [20, 60, 100, 140, 180, 220];
    for signal in buffer.split_terminator("\n") {
        let instruction = &signal[..4];
        if instruction == "noop" {
            cycle += 1;
        } else {
            let value: i32 = signal[5..].parse().unwrap();
            cycle += 1;
            if strength_cycles.contains(&cycle) {
                strength += cycle * register;
            }
            cycle += 1;
            register += value;
        }

        if strength_cycles.contains(&cycle) {
            strength += cycle * register;
        }
    }

    println!("{strength}");
}
