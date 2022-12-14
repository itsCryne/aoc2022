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

use std::{env, fmt::Display};
use days::*;
use util::get_puzzle_input;

fn main() {
    let day_vec: Vec<[fn(&str) -> Box<dyn Display>; 2]> = vec![
        [day_01::a, day_01::b],
        [day_02::a, day_02::b],
        [day_03::a, day_03::b],
        [day_04::a, day_04::b],
        [day_05::a, day_05::b],
        [day_06::a, day_06::b],
        [day_07::a, day_07::b],
        [day_08::a, day_08::b],
        [day_09::a, day_09::b],
        [day_10::a, day_10::b],
        [day_11::a, day_11::b],
        [day_12::a, day_12::b],
        [day_13::a, day_13::b],
        [day_14::a, day_14::b]
        ];

    let args: Vec<String> = env::args().collect();
    let mut day_min = 0;
    if args.len() >= 2 {
        day_min = args[1].parse().unwrap();
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
