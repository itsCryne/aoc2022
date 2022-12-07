use std::{fmt::Display, collections::HashMap};

pub fn a(input: &str) -> Box<dyn Display> {
    let mut parent_directories = Vec::new();
    let mut directory_sizes = HashMap::new();

    for line in input.lines() {
        let mut split = line.split(" ");
        match split.next().unwrap() {
            "$" => {
                if split.next().unwrap() == "cd" {
                    match split.next().unwrap() {
                        ".." => {
                            parent_directories.pop();
                            continue;
                        }
                        directory => {
                            parent_directories.push(directory);
                            continue;
                        }
                    }
                }
            },
            "dir" => (),
            number => {
                let borrowed_dirs = &parent_directories;
                for i in 0..borrowed_dirs.len()+1 {
                    let dir_name = borrowed_dirs.split_at(i).0.join(".");
                    *directory_sizes.entry(dir_name).or_insert(0) += u64::from_str_radix(number, 10).unwrap()
                }
            }
        }
    }

    Box::new(directory_sizes.drain().filter_map(|(_,v)| if v <= 100_000 {Some(v)} else {None}).sum::<u64>())
}

pub fn b(input: &str) -> Box<dyn Display> {
    let mut parent_directories = Vec::new();
    let mut directory_sizes = HashMap::new();

    for line in input.lines() {
        let mut split = line.split(" ");
        match split.next().unwrap() {
            "$" => {
                if split.next().unwrap() == "cd" {
                    match split.next().unwrap() {
                        ".." => {
                            parent_directories.pop();
                            continue;
                        }
                        directory => {
                            parent_directories.push(directory);
                            continue;
                        }
                    }
                }
            },
            "dir" => (),
            number => {
                let borrowed_dirs = &parent_directories;
                for i in 0..borrowed_dirs.len()+1 {
                    let dir_name = borrowed_dirs.split_at(i).0.join(".");
                    *directory_sizes.entry(dir_name).or_insert(0) += u64::from_str_radix(number, 10).unwrap()
                }
            }
        }
    }

    let free_space = 70000000 - directory_sizes.get("/").unwrap();
    let deletion_size = 30000000 - free_space;

    Box::new(directory_sizes.drain().filter_map(|(_,v)| if v >= deletion_size {Some(v)} else {None}).min().unwrap())
}
