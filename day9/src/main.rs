use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from(string: &str) -> Self {
        match string {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid input"),
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point(i32, i32);

impl Point {
    fn do_move(&self, direction: &Direction) -> Point{
        match direction {
            Direction::Up => Point(self.0 + 1, self.1),
            Direction::Down => Point(self.0 - 1, self.1),
            Direction::Left => Point(self.0, self.1 - 1),
            Direction::Right => Point(self.0, self.1 + 1),
        }
    }

    fn move_delta(&self, delta: &(i32, i32)) -> Point{
        Point(self.0 + delta.0, self.1 + delta.1)
    }

    fn distance_from(&self, point: &Point) -> i32 {
        let diff = self.0.abs_diff(point.0) + self.1.abs_diff(point.1);
        diff as i32
    }

    fn lies_in_row(&self, row: i32) -> bool {
        self.0 == row
    }

    fn lies_in_col(&self, col: i32) -> bool {
        self.1 == col
    }
}

fn move_rope(rope: &mut Vec<Point>, direction: &Direction) {
    rope[0] = rope[0].do_move(direction);
    for i in 1..rope.len() {
        let ahead_point = &rope[i - 1];
        let point = &rope[i];
        let distance = point.distance_from(ahead_point);
        
        if distance == 2
            && (point.lies_in_row(ahead_point.0) || point.lies_in_col(ahead_point.1))
        {
            rope[i] = point.move_delta(&(
                (ahead_point.0 - point.0).signum(),
                (ahead_point.1 - point.1).signum(),
            ));
            continue;
        }

        if distance >= 3 {
            rope[i] = point.move_delta(&(
                (ahead_point.0 - point.0).signum(),
                (ahead_point.1 - point.1).signum(),
            ));
            continue;
        }
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let rope_length: usize = 10;
    let rope = &mut Vec::<Point>::with_capacity(rope_length);
    rope.resize_with(rope_length, || Point(0,0));

    let mut tail = HashSet::<Point>::new();

    for command in buffer.split_terminator("\n") {
        let direction = Direction::from(&command[..1]);
        let times: i32 = (&command[2..]).parse().unwrap();
        for _ in 0..times {
            move_rope(rope, &direction);
            tail.insert(*rope.last().unwrap());
        }
    }

    println!("{}", tail.len());
}
