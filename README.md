# [Advent of Code 2022](https://adventofcode.com/2022/) solutions

## General pieces of information
This repositroy contains my solutions to the Advent of Code 2022 puzzles.
I upload the sourcecode "as is" i.e. I do not modify it once I have solved the respective puzzles.

Thus, the code in this repository is in the best case unoptimized and in the worst case an utter mess.
My plan is to revise the solutions later on and add the polished code to another branch

## Setup
- Create a file named "session_key.txt" in the "src" folder.
- Get your session key
    - Press CTRL + Shift + I
    - Switch to the "Network" tab
    - Visit (or reload) https://adventofcode.com/ (& make sure you're logged in)
    - Select a request by clicking on it
    - On the right side, select "Cookies"
    - Copy the part between the quotes.
        - The session key should be 96 Hex-Characters
        - **WARNING**: DO NOT SHARE THIS KEY. NOT EVEN WITH YOUR DOG!
- Paste this string (and only this string!) into the "session_key.txt" file
- You can now [run the project with Cargo](https://doc.rust-lang.org/cargo/commands/cargo-run.html) (you must have [Rust installed](https://www.rust-lang.org/tools/install)).
- To schedule multiple runs (for averaging the times), add the number of runs just after the cargo command

If the request fails with "500 Internal Server Error", you entered a wrong session key.