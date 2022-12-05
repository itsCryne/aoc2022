use std::fmt::Display;

pub fn a(input: &str) -> Box<dyn Display> {
    let lines = input.lines();
    let mut score = 0;

    for line in lines {
        match line {
            "A X" => score += 4,
            "A Y" => score += 8,
            "A Z" => score += 3,
            "B X" => score += 1,
            "B Y" => score += 5,
            "B Z" => score += 9,
            "C X" => score += 7,
            "C Y" => score += 2,
            "C Z" => score += 6,
            _ => score += 0
        }
    }

    return Box::new(score);
}
pub fn b(input: &str) -> Box<dyn Display> {
    let lines = input.lines();
    let mut score = 0;

    for line in lines {
        match line {
            "A X" => score += 3,
            "A Y" => score += 4,
            "A Z" => score += 8,
            "B X" => score += 1,
            "B Y" => score += 5,
            "B Z" => score += 9,
            "C X" => score += 2,
            "C Y" => score += 6,
            "C Z" => score += 7,
            _ => score += 0
        }
    }

    return Box::new(score);
}

