use std::{fmt::Display, collections::HashSet};

fn distance_too_large(head: (i32, i32), tail: (i32, i32)) -> bool {
    // sqrt(2) is approximately 1.4
    (((head.0 - tail.0).pow(2) + (head.1 - tail.1).pow(2)) as f64).powf(0.5) > 1.5
}

pub fn a(input: &str) -> Box<dyn Display> {
    let mut visited_tail_positions = HashSet::from([((0, 0))]);
    let mut head_pos = (0, 0);
    let mut tail_pos = (0,0);

    let moves = input.lines()
        .map(|l| l.trim().split_once(" ").unwrap())
        .map(|(d, l)| (d, i32::from_str_radix(l, 10).unwrap()))
        .collect::<Vec<(&str, i32)>>();

    for (direction, distance) in moves {
        match direction {
            "U" => {
                for _ in 0..distance {
                    head_pos.1 += 1;
                    if distance_too_large(head_pos, tail_pos) {
                        tail_pos = (head_pos.0, head_pos.1-1);
                        visited_tail_positions.insert(tail_pos);
                    }
                }
            }
            "R" => {
                for _ in 0..distance {
                    head_pos.0 += 1;
                    if distance_too_large(head_pos, tail_pos) {
                        tail_pos = (head_pos.0-1, head_pos.1);
                        visited_tail_positions.insert(tail_pos);
                    }
                }
            }
            "D" => {
                for _ in 0..distance {
                    head_pos.1 -= 1;
                    if distance_too_large(head_pos, tail_pos) {
                        tail_pos = (head_pos.0, head_pos.1+1);
                        visited_tail_positions.insert(tail_pos);
                    }
                }
            }
            "L" => {
                for _ in 0..distance {
                    head_pos.0 -= 1;
                    if distance_too_large(head_pos, tail_pos) {
                        tail_pos = (head_pos.0+1, head_pos.1);
                        visited_tail_positions.insert(tail_pos);
                    }
                }
            }
            _ => ()
        }
    }
    Box::new(visited_tail_positions.len())
}

// Thank you, Lena
pub fn b(input: &str) -> Box<dyn Display> {
    let mut visited_tail_positions = HashSet::from([((0, 0))]);

    let moves = input.lines()
        .map(|l| l.trim().split_once(" ").unwrap())
        .map(|(d, l)| (d, i32::from_str_radix(l, 10).unwrap()))
        .collect::<Vec<(&str, i32)>>();

    let mut head_positions = vec![(0,0)];
    for m in moves {
        for _ in 0..m.1 {
            let last_pos = head_positions[head_positions.len()-1];

            match m.0 {
                "U" => head_positions.push((last_pos.0, last_pos.1 + 1)),
                "D" => head_positions.push((last_pos.0, last_pos.1 - 1)),
                "R" => head_positions.push((last_pos.0 + 1, last_pos.1)),
                "L" => head_positions.push((last_pos.0 - 1, last_pos.1)),
                _ => unreachable!()
            }
        }
    }
    let mut tail_positions = vec![(0,0)];

    let l = 9;
    for i in 0..l {
        for hpos in head_positions {
            let tpos = tail_positions[tail_positions.len() - 1];

            match (hpos.0 - tpos. 0, hpos.1 - tpos.1) {
                (2, 2) => tail_positions.push((tpos.0 + 1, tpos.1 + 1)),
                (2, -2) => tail_positions.push((tpos.0 + 1, tpos.1 - 1)),
                (-2, -2) => tail_positions.push((tpos.0 - 1, tpos.1 - 1)),
                (-2, 2) => tail_positions.push((tpos.0 - 1, tpos.1 + 1)),

                (2, 0) => tail_positions.push((tpos.0 + 1, tpos.1)),
                (-2, 0) => tail_positions.push((tpos.0 - 1, tpos.1)),
                (0, -2) => tail_positions.push((tpos.0, tpos.1 - 1)),
                (0, 2) => tail_positions.push((tpos.0, tpos.1 + 1)),

                (2, 1) => tail_positions.push((tpos.0 + 1, tpos.1 + 1)),
                (1, 2) => tail_positions.push((tpos.0 + 1, tpos.1 + 1)),

                (-1, 2) => tail_positions.push((tpos.0 - 1, tpos.1 + 1)),
                (-2, 1) => tail_positions.push((tpos.0 - 1, tpos.1 + 1)),

                (-2, -1) => tail_positions.push((tpos.0 - 1, tpos.1 - 1)),
                (-1, -2) => tail_positions.push((tpos.0 - 1, tpos.1 - 1)),

                (1, -2) => tail_positions.push((tpos.0 + 1, tpos.1 - 1)),
                (2, -1) => tail_positions.push((tpos.0 + 1, tpos.1 - 1)),
                _ => tail_positions.push((tpos.0, tpos.1))
            }
        }
        if i == l-1 {
            for m in &tail_positions {
                visited_tail_positions.insert(*m);
            }
        }
        head_positions = tail_positions;
        tail_positions = vec![(0,0)];
    }

    Box::new(visited_tail_positions.len())
}
