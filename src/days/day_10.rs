use std::{fmt::Display};

pub fn a(input: &str) -> Box<dyn Display> {
    let mut register_x = 1;
    let mut signal_strength_sum = 0;
    let mut cycle = 1_i64;

    for line in input.lines() {
        cycle += 1;

        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            signal_strength_sum += cycle * register_x;
        }

        if let Some((_, count)) = line.split_once(" ") {
            register_x += i64::from_str_radix(count, 10).unwrap();
            cycle += 1;

            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                signal_strength_sum += cycle * register_x;
            }
        }

    }

    Box::new(signal_strength_sum)
}

pub fn b(input: &str) -> Box<dyn Display> {
    let mut register_x = 1;
    let mut cycle = 0_i64;
    let mut crt_lines = vec![vec![]; 6];

    for line in input.lines() {
        cycle += 1;

        let crt_vpos = (cycle-1) / 40;
        let crt_hpos = cycle - ((cycle-1) / 40) * 40;

        if [crt_hpos - 1, crt_hpos, crt_hpos + 1].contains(&(register_x)) {
            crt_lines[crt_vpos as usize].push('#');
        } else {
            crt_lines[crt_vpos as usize].push('.');
        }

        if let Some((_, count)) = line.split_once(" ") {

            cycle += 1;
            register_x += i64::from_str_radix(count, 10).unwrap();

            let crt_vpos = (cycle-1) / 40;
            let crt_hpos = cycle - ((cycle-1) / 40) * 40;
            if [crt_hpos - 1, crt_hpos, crt_hpos + 1].contains(&(register_x)) {
                crt_lines[crt_vpos as usize].push('#');
            } else {
                crt_lines[crt_vpos as usize].push('.');
            }

        }
    }

    let display = crt_lines.into_iter().map(|l| l.into_iter().collect::<String>()).collect::<Vec<String>>();

    Box::new(String::from("\n") + &display.join("\n"))
}
