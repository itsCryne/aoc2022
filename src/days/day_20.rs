use std::fmt::Display;

fn modulo(a: i64, b: usize) -> usize {
    let b = b as i64;
    (((a % b) + b) % b) as usize
}

pub fn a(input: &str) -> Box<dyn Display> {
    let mut encrypted = input.lines().map(|l| (i64::from_str_radix(l, 10).unwrap(), false)).collect::<Vec<_>>();

    while let Some(pos) = encrypted.iter().position(|e| e.1 == false) {
        let elem = (encrypted.remove(pos).0, true);
        let new_pos = modulo(elem.0 + pos as i64, encrypted.len());
        encrypted.insert(new_pos, elem);
    }

    let zero_pos = encrypted.iter().position(|e| e.0 == 0).unwrap();
    let one_pos = modulo(1000 + zero_pos as i64, encrypted.len());
    let two_pos = modulo(2000 + zero_pos as i64, encrypted.len());
    let three_pos = modulo(3000 + zero_pos as i64, encrypted.len());
    
    let result = encrypted[one_pos].0 + encrypted[two_pos].0 +encrypted[three_pos].0;

    Box::new(result)
}

pub fn b(input: &str) -> Box<dyn Display> {
    let mut encrypted = input.lines().map(|l| 811589153 * i64::from_str_radix(l, 10).unwrap()).enumerate().collect::<Vec<_>>();

    for _ in 0..10 {
        for i in 0..encrypted.len() {
            let pos = encrypted.iter().position(|n| n.0 == i).unwrap();
            let elem = encrypted.remove(pos);
            let new_pos = modulo(elem.1 + pos as i64, encrypted.len());
            encrypted.insert(new_pos, elem);
        }
    }

    let zero_pos = encrypted.iter().position(|e| e.1 == 0).unwrap();
    let one_pos = modulo(1000 + zero_pos as i64, encrypted.len());
    let two_pos = modulo(2000 + zero_pos as i64, encrypted.len());
    let three_pos = modulo(3000 + zero_pos as i64, encrypted.len());
    
    let result = encrypted[one_pos].1 + encrypted[two_pos].1 +encrypted[three_pos].1;
    
    Box::new(result)
}
