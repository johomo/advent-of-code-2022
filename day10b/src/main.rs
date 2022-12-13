use std::io;
use std::io::prelude::*;

fn sprite_overlaps_crt(crt: i32, register: i32) -> bool {
    (crt % 40).abs_diff(register) <= 1
}

fn print_screen(screen: &[bool; 240]) {
    for row in 0..6 {
        for col in 0..40 {
            let value = screen[40 * row + col];
            if value {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut crt_screen: [bool; 240] = [false; 240];
    let mut crt_cycle: usize = 0;
    let mut register: i32 = 1;
    for signal in buffer.split_terminator("\n") {
        let instruction = &signal[..4];
        if instruction == "noop" {
            crt_cycle += 1;
        } else {
            let value: i32 = signal[5..].parse().unwrap();
            crt_cycle += 1;
            if crt_cycle >= crt_screen.len() {
                break;
            }
            crt_screen[crt_cycle] = sprite_overlaps_crt(crt_cycle as i32, register);
            crt_cycle += 1;
            register += value;
        }
        if crt_cycle >= crt_screen.len() {
            break;
        }
        crt_screen[crt_cycle] = sprite_overlaps_crt(crt_cycle as i32, register);
    }

    print_screen(&crt_screen);
}
