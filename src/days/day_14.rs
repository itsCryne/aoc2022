use std::{fmt::Display, collections::HashSet};

pub fn a(input: &str) -> Box<dyn Display> {
    let mut blocked_positions = HashSet::new();

    for line in input.lines() {
        for row in line.split(" -> ").collect::<Vec<_>>().windows(2) {
            let (x1, y1) = row[0].split_once(",").unwrap();
            let (x2, y2) = row[1].split_once(",").unwrap();

            let x1 = u64::from_str_radix(x1, 10).unwrap();
            let y1 = u64::from_str_radix(y1, 10).unwrap();
            let x2 = u64::from_str_radix(x2, 10).unwrap();
            let y2 = u64::from_str_radix(y2, 10).unwrap();

            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    blocked_positions.insert((x1, y));
                }
            } else if y1 == y2 {
                for x in x1.min(x2)..=x1.max(x2) {
                    blocked_positions.insert((x, y1));
                }
            } else {
                unreachable!();
            }    
        }
    }

    let max_y = blocked_positions.iter().map(|(_, y)| y).max().unwrap().clone();
    let mut settled_sand_pieces = 0;

    'sandloop: loop {
        let mut sand_position = (500, 0);
        for y in 0.. {
            if y > max_y {
                break 'sandloop;
            }

            if !blocked_positions.contains(&(sand_position.0, sand_position.1+1)) {
                sand_position = (sand_position.0, sand_position.1+1);
            } else if !blocked_positions.contains(&(sand_position.0-1, sand_position.1+1)) {
                sand_position = (sand_position.0-1, sand_position.1+1);
            } else if !blocked_positions.contains(&(sand_position.0+1, sand_position.1+1)) {
                sand_position = (sand_position.0+1, sand_position.1+1);
            } else {
                settled_sand_pieces += 1;
                blocked_positions.insert(sand_position);
                break;
            }
        }
    }

    Box::new(settled_sand_pieces)
}

pub fn b(input: &str) -> Box<dyn Display> {
    let mut blocked_positions = HashSet::new();

    for line in input.lines() {
        for row in line.split(" -> ").collect::<Vec<_>>().windows(2) {
            let (x1, y1) = row[0].split_once(",").unwrap();
            let (x2, y2) = row[1].split_once(",").unwrap();

            let x1 = u64::from_str_radix(x1, 10).unwrap();
            let y1 = u64::from_str_radix(y1, 10).unwrap();
            let x2 = u64::from_str_radix(x2, 10).unwrap();
            let y2 = u64::from_str_radix(y2, 10).unwrap();

            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    blocked_positions.insert((x1, y));
                }
            } else if y1 == y2 {
                for x in x1.min(x2)..=x1.max(x2) {
                    blocked_positions.insert((x, y1));
                }
            } else {
                unreachable!();
            }    
        }
    }

    let max_y = blocked_positions.iter().map(|(_, y)| y).max().unwrap().clone();
    let mut settled_sand_pieces = 0;

    'sandloop: loop {
        let mut sand_position = (500, 0);
        for y in 0.. {
            if y == max_y + 1 {
                settled_sand_pieces += 1;
                blocked_positions.insert(sand_position);
                break;
            }

            if !blocked_positions.contains(&(sand_position.0, sand_position.1+1)) {
                sand_position = (sand_position.0, sand_position.1+1);
            } else if !blocked_positions.contains(&(sand_position.0-1, sand_position.1+1)) {
                sand_position = (sand_position.0-1, sand_position.1+1);
            } else if !blocked_positions.contains(&(sand_position.0+1, sand_position.1+1)) {
                sand_position = (sand_position.0+1, sand_position.1+1);
            } else {
                settled_sand_pieces += 1;
                blocked_positions.insert(sand_position);

                if sand_position == (500, 0) {
                    break 'sandloop;
                }
                break;
            }
        }
    }

    Box::new(settled_sand_pieces)
}
