use std::fmt::Display;

pub fn a(input: &str) -> Box<dyn Display> {
    let lines: Vec<&str> = input.lines().collect();
    let mut cals = vec![0;lines.len()];
    let mut elf = 0;

    for line in lines {
        if line == "" {
            elf += 1;
        } else {
            cals[elf] += usize::from_str_radix(line, 10).unwrap();
        }
    }

    return Box::new(*cals.iter().max().unwrap());
}
pub fn b(input: &str) -> Box<dyn Display> {
    let lines: Vec<&str> = input.lines().collect();
    let mut cals = vec![0;lines.len()];
    let mut elf = 0;

    for line in lines {
        if line == "" {
            elf += 1;
        } else {
            cals[elf] += usize::from_str_radix(line, 10).unwrap();
        }
    }

    cals.sort_unstable_by(|a, b| b.cmp(a));

    return Box::new(cals[0] + cals[1] + cals[2]);
}

