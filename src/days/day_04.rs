use std::fmt::Display;

fn to_bitfield(start: u8, end: u8) -> u128 {
    let mut res: u128 = 0;
    for i in start..=end {
        res |= 1 << i
    }
    res
}

pub fn a(input: &str) -> Box<dyn Display> {
    let mut res = 0;
    for line in input.lines() {
        let (e1, e2) = line.split_once(",").unwrap();

        let (se1, ee1) = e1.split_once("-").unwrap();
        let (se2, ee2) = e2.split_once("-").unwrap();

        let be1 = to_bitfield(u8::from_str_radix(se1, 10).unwrap(), u8::from_str_radix(ee1, 10).unwrap());
        let be2 = to_bitfield(u8::from_str_radix(se2, 10).unwrap(), u8::from_str_radix(ee2, 10).unwrap());

        if be1 & be2 == be1 || be1 & be2 == be2 {
            res += 1;
        }

    }

    Box::new(res)
}

pub fn b(input: &str) -> Box<dyn Display> {
    let mut res = 0;
    for line in input.lines() {
        let (e1, e2) = line.split_once(",").unwrap();

        let (se1, ee1) = e1.split_once("-").unwrap();
        let (se2, ee2) = e2.split_once("-").unwrap();

        let be1 = to_bitfield(u8::from_str_radix(se1, 10).unwrap(), u8::from_str_radix(ee1, 10).unwrap());
        let be2 = to_bitfield(u8::from_str_radix(se2, 10).unwrap(), u8::from_str_radix(ee2, 10).unwrap());

        if be1 & be2 != 0 {
            res += 1;
        }

    }

    Box::new(res as usize)
}

