use std::io;
use std::io::prelude::*;

fn parse_grid(buffer: &str) -> Vec<Vec<u32>> {
    buffer
        .split_terminator("\n")
        .map(|buffer| {
            buffer
                .chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn compute_max_scenic_score(grid: &Vec<Vec<u32>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut max_scenic_score = 0;
    for row in 0..rows {
        for col in 0..cols {
            let scenic_score = scenic_score(row, col, &grid);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }
    return max_scenic_score;
}

fn scenic_score(row: usize, col: usize, grid: &Vec<Vec<u32>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let is_edge_tree = row == 0 || row == (rows - 1) || col == 0 || col == (cols - 1);
    if is_edge_tree {
        return 0;
    }

    let tree_size = grid[row][col];
    let mut score_left = 0;
    for (tree_count, j) in (0..col).rev().enumerate() {
        score_left = tree_count + 1;
        if grid[row][j] >= tree_size {
            break;
        }
    }

    let mut score_right = 0;
    for (tree_count, j) in (col + 1..cols).enumerate() {
        score_right = tree_count + 1;
        if grid[row][j] >= tree_size {
            break;
        }
    }

    let mut score_up = 0;
    for (tree_count, i) in (0..row).rev().enumerate() {
        score_up = tree_count + 1;
        if grid[i][col] >= tree_size {
            break;
        }
    }

    let mut score_down = 0;
    for (tree_count, i) in (row + 1..rows).enumerate() {
        score_down = tree_count + 1;
        if grid[i][col] >= tree_size {
            break;
        }
    }

    return score_left * score_right * score_up * score_down;
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let grid = parse_grid(&buffer);
    let max_scenic_score = compute_max_scenic_score(&grid);

    println!("{max_scenic_score}");
}
