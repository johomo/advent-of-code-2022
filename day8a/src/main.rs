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

fn count_visible_trees(grid: &Vec<Vec<u32>>) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visible_trees: u32 = 0;

    for row in 0..rows {
        for col in 0..cols {
            if tree_is_visible(row, col, &grid) {
                visible_trees += 1;
            }
        }
    }
    return visible_trees;
}

fn tree_is_visible(row: usize, col: usize, grid: &Vec<Vec<u32>>) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();

    let is_edge_tree = row == 0 || row == (rows - 1) || col == 0 || col == (cols - 1);
    if is_edge_tree {
        return true;
    }

    let tree_size = grid[row][col];
    let mut is_visible_from_left = true;
    for j in 0..col {
        if grid[row][j] >= tree_size {
            is_visible_from_left = false;
            break;
        }
    }

    let mut is_visible_from_right = true;
    for j in col + 1..cols {
        if grid[row][j] >= tree_size {
            is_visible_from_right = false;
            break;
        }
    }

    let mut is_visible_from_top = true;
    for i in 0..row {
        if grid[i][col] >= tree_size {
            is_visible_from_top = false;
            break;
        }
    }

    let mut is_visible_from_bottom = true;
    for i in row + 1..rows {
        if grid[i][col] >= tree_size {
            is_visible_from_bottom = false;
            break;
        }
    }

    return is_visible_from_bottom
        || is_visible_from_top
        || is_visible_from_left
        || is_visible_from_right;
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let grid = parse_grid(&buffer);
    let visible_trees = count_visible_trees(&grid);

    println!("{visible_trees}");
}
