const SIZE: usize = 99;

use std::{fmt::Display};

enum Direction {
    Up,
    Down,
    Right,
    Left
}

fn is_visible(x: usize, y: usize, map: &mut [[u8;SIZE];SIZE], vmap: &mut [[Option<bool>;SIZE];SIZE], direction: Direction) -> bool {
    match direction {
        Direction::Down => {
            if x == SIZE-1 {
                return true;
            }

            for check_x in x+1..SIZE {
                if vmap[check_x][y] == Some(true) {
                    return map[check_x][y] < map[x][y]
                }
                if map[check_x][y] >= map[x][y] {
                    return false;
                }
            }
            return true;
        }
        Direction::Up => {
            if x == 0 {
                return true;
            }

            for check_x in (0..x).rev() {
                if vmap[check_x][y] == Some(true) {
                    return map[check_x][y] < map[x][y]
                }
                if map[check_x][y] >= map[x][y] {
                    return false;
                }
            }
            return true;
        },
        Direction::Right => {
            if y == SIZE-1 {
                return true;
            }

            for check_y in y+1..SIZE {
                if vmap[x][check_y] == Some(true) {
                    return map[x][check_y] < map[x][y]
                }
                if map[x][check_y] >= map[x][y] {
                    return false;
                }
            }
            return true;
        }
        Direction::Left => {
            if y == 0 {
                return true;
            }

            for check_y in (0..y).rev() {
                if vmap[x][check_y] == Some(true) {
                    return map[x][check_y] < map[x][y]
                }
                if map[x][check_y] >= map[x][y] {
                    return false;
                }
            }
            return true;
        }
    }
}

// In dire need of cleanup. But it works.
pub fn a(input: &str) -> Box<dyn Display> {
    let map = &mut [[0;SIZE];SIZE];

    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            map[x][y] = char.to_digit(10).unwrap() as u8;
        }
    }

    let umap = &mut [[None;SIZE];SIZE];
    let lmap = &mut [[None;SIZE];SIZE];
    let dmap = &mut [[None;SIZE];SIZE];
    let rmap = &mut [[None;SIZE];SIZE];



    let mut counter = 0;
    (0..SIZE).for_each(|x| (0..SIZE).for_each(|y| {
        umap[x][y] = Some(is_visible(x, y, map, rmap, Direction::Up));
    }));
    (0..SIZE).for_each(|x| (0..SIZE).for_each(|y| {
        lmap[x][y] = Some(is_visible(x, y, map, rmap, Direction::Left));
    }));
    (0..SIZE).for_each(|x| (0..SIZE).for_each(|y| {
        dmap[x][y] = Some(is_visible(x, y, map, rmap, Direction::Down));
    }));
    (0..SIZE).for_each(|x| (0..SIZE).for_each(|y| {
        rmap[x][y] = Some(is_visible(x, y, map, rmap, Direction::Right));
    }));

    let vmap = &mut [[None;SIZE];SIZE];
    (0..SIZE).for_each(|x| (0..SIZE).for_each(|y| {
        let vis = [&umap, &lmap, &dmap, &rmap].iter().any(|tmap| tmap[x][y] == Some(true));
        vmap[x][y] = Some(vis);
        counter += vis as u8 as u64;
    }));

    Box::new(counter)
    
}

pub fn b(input: &str) -> Box<dyn Display> {
    let map = &mut [[0;SIZE];SIZE];

    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            map[x][y] = char.to_digit(10).unwrap() as u8;
        }
    }

    let mut scenic_scores: Vec<u64> = vec![];

    for x in 0..SIZE {
        for y in 0..SIZE {
            let mut ucounter = 0;
            let mut lcounter = 0;
            let mut dcounter = 0;
            let mut rcounter = 0;

            for check_x in x+1..SIZE {
                dcounter += 1;
                if map[check_x][y] >= map[x][y] {
                    break;
                }
            }
        
            for check_x in (0..x).rev() {
                ucounter += 1;
                if map[check_x][y] >= map[x][y] {
                    break;
                }
            }
        
            for check_y in y+1..SIZE {
                rcounter += 1;
                if map[x][check_y] >= map[x][y] {
                    break;
                }
            }
        
            for check_y in (0..y).rev() {
                lcounter += 1;
                if map[x][check_y] >= map[x][y] {
                    break;
                }
            }
            
            scenic_scores.push([ucounter, lcounter, dcounter, rcounter].iter().product());
        }
    }

    Box::new(scenic_scores.iter().max().unwrap().clone())
}
