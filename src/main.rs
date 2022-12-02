/*
    aoc2022 - solutions for the "Advent of Code 2022" in rust
    Copyright (C) 2022 itsCryne <github@cryne.me>

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

mod days;
mod util;

use std::env;
use days::*;
use util::get_puzzle_input;

fn main() {
    //TODO: Optimize this with generics so we can use appropiately sized types on every function
    let day_vec = vec![
        [day_01::a, day_01::b],
        [day_02::a, day_02::b]
    ];

    let args: Vec<String> = env::args().collect();
    let mut day_min = 0;
    if args.len() >= 2 {
        day_min = args[2].parse().unwrap();
        println!("Starting at day {}", day_min);
    }


    for (day, tasks) in day_vec.iter().enumerate() {
        if day + 1 < day_min {
            continue;
        }
        let input = get_puzzle_input((day + 1) as i8);
        let a_result = tasks[0](&input);
        let b_result = tasks[1](&input);

        println!("Day {:02}:\n A: {}\n B: {}", day+1,  a_result, b_result);
    }
}
