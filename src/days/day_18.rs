use std::{fmt::Display, collections::HashSet};

pub fn a(input: &str) -> Box<dyn Display> {
    let mut grid = HashSet::new();
    let mut counter = 0;

    for line in input.lines() {
        let coords = line.split(",").map(|c| i32::from_str_radix(c, 10).unwrap()).collect::<Vec<i32>>();
        grid.insert((coords[0], coords[1], coords[2]));
    }

    for (x, y, z) in grid.clone() {
        counter += !grid.contains(&(x-1, y, z)) as u32;
        counter += !grid.contains(&(x+1, y, z)) as u32;
        counter += !grid.contains(&(x, y-1, z)) as u32;
        counter += !grid.contains(&(x, y+1, z)) as u32;
        counter += !grid.contains(&(x, y, z-1)) as u32;
        counter += !grid.contains(&(x, y, z+1)) as u32;
    }

    Box::new(counter)
}

pub fn b(_input: &str) -> Box<dyn Display> {
    Box::new(0)
}
