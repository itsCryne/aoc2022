# [Advent of Code 2022](https://adventofcode.com/2022/) solutions

### What this repository is
This repository contains solutions to [Eric Wastl](https://twitter.com/ericwastl)s Advent of Code 2022. Thank you, Eric!

While the code aims to be as fast as possible (the goal is to complete all puzzles of 2022 in under one second),
I often lack the time and/or knowledge to optimize it properly.

### What this repository is **not**
I'm currently learning Rust, so don't expect the solutions to be idiomatic.

This is not the "best" or maybe not even a "good" solution. Most often it "just works".<br>
Please do not use this repository to learn how to write good or fast code.

### Setup
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