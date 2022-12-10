use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug)]
enum Fd {
    File(usize),
    Directory(usize),
}

impl Fd {
    fn size(&self) -> usize {
        *match self {
            Fd::File(size) => size,
            Fd::Directory(size) => size,
        }
    }
}

fn parse_directories(buffer: &str) -> HashMap<PathBuf, Fd> {
    let mut dirs = HashMap::<PathBuf, Fd>::new();
    let mut current_directory = PathBuf::from("/");
    dirs.insert(current_directory.clone(), Fd::Directory(0));

    for line in buffer.split_terminator("\n") {
        if line.starts_with("$ cd") {
            let dir_name = &line[5..];
            current_directory = match dir_name {
                ".." => current_directory.parent().unwrap().to_path_buf(),
                "/" => PathBuf::from("/"),
                _ => current_directory.join(dir_name),
            }
        } else if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("dir") {
            let dir_name = &line[4..];
            dirs.insert(current_directory.join(dir_name), Fd::Directory(0));
        } else {
            let chunks: Vec<&str> = line.split_terminator(" ").collect();
            let file_size: usize = chunks[0].parse().unwrap();
            let file_name = chunks[1];
            dirs.insert(current_directory.join(file_name), Fd::File(file_size));
        }
    }

    return dirs;
}

fn compute_directories_sizes(dirs: &mut HashMap<PathBuf, Fd>) {
    let files: Vec<PathBuf> = dirs
        .iter()
        .filter(|(_, fd)| match fd {
            Fd::File(_) => true,
            Fd::Directory(_) => false,
        })
        .map(|(path, _)| path.clone())
        .collect();

    for file_path in files.iter() {
        let fd = dirs.get(file_path).unwrap();
        let file_size = fd.size();
        let mut parent = file_path.parent().unwrap().to_path_buf();
        loop {
            let dir_size = dirs.get(&parent).unwrap().size();
            dirs.insert(parent.clone(), Fd::Directory(dir_size + file_size));
            parent = match parent.parent() {
                Some(path) => path.to_path_buf(),
                None => break,
            };
        }
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut directories = parse_directories(&buffer);
    compute_directories_sizes(&mut directories);

    // Problem a
    let threshold: usize = 100000;
    let total_size: usize = directories
        .values()
        .map(|fd| match fd {
            Fd::Directory(size) => *size,
            _ => 0,
        })
        .filter(|&size| (size > 0) && (size <= threshold))
        .sum();

    println!("Problem a: {total_size}");

    // Problem b
    let disk_size: usize = 70000000;
    let disk_occupied = directories.get(&PathBuf::from("/")).unwrap().size();
    let disk_free = disk_size - disk_occupied;
    let disk_update_size: usize = 30000000;
    if disk_free >= disk_update_size {
        println!("No need to delete any directory");
        return;
    }
    let must_free = disk_update_size - disk_free;
    let delete_directory_size = directories
        .values()
        .map(|fd| match fd {
            Fd::Directory(size) => *size,
            _ => 0,
        })
        .filter(|&size| size >= must_free)
        .min()
        .unwrap();

    println!("Problem b: {delete_directory_size}");
}
