use std::fs::read_to_string;
use ureq;
use ureq::Error;

pub(crate) fn get_puzzle_input(day: i8) -> String {
    if let Ok(input) = std::fs::read_to_string(format!("inputs/day_{:02}.txt", day)) {
        return input;
    }

    let session_key = read_to_string("session_key.txt")
        .expect("Failed to read \"session_key.txt\". Did you create the file (at the right place)?");

    let input = match ureq::get(&*format!("https://adventofcode.com/2022/day/{}/input", day))
        .set("Cookie", &*format!("session={}", session_key))
        .set("User-Agent", "https://github.com/itsCryne/aoc2022 by paddi@cryne.me")
        .call() {
        Ok(response) => response.into_string().unwrap(),
        Err(Error::Status(code, response)) => {
            panic!("Server returned an error! {} {}. Did you input your session key correctly?", code, response.into_string().unwrap())
        },
        Err(_) => {
            panic!("There was an error while performing the request. Please check your internet connection and try again later.")
        }
    };

    if !std::path::Path::new("inputs").exists() {
        std::fs::create_dir("inputs").unwrap();
    }
    if !std::path::Path::new(&format!("inputs/day_{:02}.txt", day)).exists() {
        std::fs::write(format!("inputs/day_{:02}.txt", day), input.clone()).unwrap();
    }

    return input;
}