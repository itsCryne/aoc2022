use std::fmt::Display;

fn build_stacks(stack_lines: &str) -> Vec<Vec<char>> {
    let mut stack_lines_iter = stack_lines.lines().rev();
    let numbers = stack_lines_iter.next().unwrap();

    let mut stacks = vec![vec![]; (numbers.len() + 1) / 4];
    for line in stack_lines_iter {
        for (n, c) in line.char_indices() {
            if n > 0 && (n-1) & 3 == 0 && c.is_ascii_uppercase() {
                stacks[n / 4].push(c);
            }
        }
    }
    
    stacks
}


pub fn a(input: &str) -> Box<dyn Display> {
    let (stack_lines, move_lines) = input.split_once("\n\n").unwrap();
    let mut stacks = build_stacks(stack_lines);

    for line in move_lines.lines() {
        let (amount, positions) = line.strip_prefix("move ").unwrap().split_once(" from ").unwrap();
        let (from, to) = positions.trim().split_once(" to ").unwrap();

        let amount = u8::from_str_radix(amount, 10).unwrap();
        let from = usize::from_str_radix(from, 10).unwrap();
        let to = usize::from_str_radix(to, 10).unwrap();

        for _ in 0..amount {
            let p = stacks[from-1].pop().unwrap();
            stacks[to-1].push(p);
        }
    }

    Box::new(stacks.iter_mut().map(|s| s.pop().unwrap_or(' ')).collect::<String>())

}

pub fn b(input: &str) -> Box<dyn Display> {
    let (stack_lines, move_lines) = input.split_once("\n\n").unwrap();
    let mut stacks = build_stacks(stack_lines);

    for line in move_lines.lines() {
        let (amount, positions) = line.strip_prefix("move ").unwrap().split_once(" from ").unwrap();
        let (from, to) = positions.trim().split_once(" to ").unwrap();

        let amount = u8::from_str_radix(amount, 10).unwrap();
        let from = usize::from_str_radix(from, 10).unwrap();
        let to = usize::from_str_radix(to, 10).unwrap();

        let mut tempstack = vec![];
        for _ in 0..amount {
            let p = stacks[from-1].pop().unwrap();
            tempstack.push(p);
        }

                    
        tempstack.reverse();
        stacks[to-1].append(&mut tempstack)
    }

    Box::new(stacks.iter_mut().map(|s| s.pop().unwrap_or(' ')).collect::<String>())
}
