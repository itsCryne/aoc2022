use std::fmt::Display;

pub fn a(input: &str) -> Box<dyn Display> {
    Box::new(input.lines()
        .map(|l| l.split_at(l.len()/2))
        .map(|(p1, p2)| p1.chars().find_map(|c| if p2.contains(c) {Some(c)} else {None}).unwrap())
        .map(|c| if c.is_lowercase() {c as u32 - 96} else {c as u32 - 40 + 2})
        .sum::<u32>() as usize)
}

pub fn b(input: &str) -> Box<dyn Display> {
    Box::new(input.lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|ls| (ls[0], ls[1], ls[2]))
        .map(|(l1, l2, l3)| l1.chars().find_map(|c| if l2.contains(c) && l3.contains(c) {Some(c)} else {None}).unwrap())
        .map(|c| if c.is_lowercase() {c as u32 - 96} else {c as u32 - 40 + 2})
        .sum::<u32>() as usize)
}

