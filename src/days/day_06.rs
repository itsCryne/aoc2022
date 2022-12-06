use std::fmt::Display;

pub fn a(input: &str) -> Box<dyn Display> {
    for (w, i) in input.as_bytes().windows(4).enumerate() {
        if i[0] != i[1] && i[0] != i[2] && i[0] != i[3]
        && i[1] != i[2] && i[1] != i[3]
        && i[2] != i[3] {
            return Box::new(w+4);
        }
    }
    unreachable!("Malformed input");

}

pub fn b(input: &str) -> Box<dyn Display> {
    'outer: for (w, i) in input.as_bytes().windows(14).enumerate() {
        for c in i {
            if i.iter().filter(|&e| e == c).count() != 1 {
                continue 'outer;
            }
        }
        return Box::new(w+14)
    }
    unreachable!("Malformed input");
}
