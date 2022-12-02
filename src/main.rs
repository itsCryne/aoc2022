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
use std::time::Instant;
use util::get_puzzle_input;

fn fmt_time(ns: u128) -> String {
    let s = ns / 1_000_000_000;
    let ms = ns / 1_000_000 - s * 1_000;
    let mu_s = ns / 1_000 - ms * 1_000 - s * 1_000_000;
    let parsed_ns = ns - mu_s * 1_000 - ms * 1_000_000 - s * 1_000_000_000;

    let mut output = format!("{}s {}ms {}µs {}ns ({}ns)", s, ms, mu_s, parsed_ns, ns);
    for char in output.clone().chars() {
        if char == '0' || char == ' ' || char.is_alphabetic() {
            output.remove(0);
        } else {
            break;
        }
    }
    output
}

fn main() {
    //TODO: Optimize this with generics so we can use appropiately sized types on every function
    let day_vec = vec![
        [day_01::a, day_01::b],
        [day_02::a, day_02::b]
    ];

    let args: Vec<String> = env::args().collect();
    let mut runs = 1;
    let mut day_min = 0;
    if args.len() == 2 {
        runs = args[1].parse().unwrap();
        println!("\n{} runs scheduled", runs);
    } else if args.len() == 3 {
        day_min = args[2].parse().unwrap();
        println!("Starting at day {}", day_min);
    }


    let mut cum_time: u128 = 0;

    for (day, tasks) in day_vec.iter().enumerate() {
        if day + 1 < day_min {
            continue;
        }
        let input = get_puzzle_input((day + 1) as i8);
        let a_result = tasks[0](&input);
        let b_result = tasks[1](&input);

        let mut a_cum_time = 0;
        let mut b_cum_time = 0;

        for _ in 0..runs {
            let a_start_time = Instant::now();
            tasks[0](&input);
            a_cum_time += a_start_time.elapsed().as_nanos();

            let b_start_time = Instant::now();
            tasks[1](&input);
            b_cum_time += b_start_time.elapsed().as_nanos();
        }

        cum_time += a_cum_time + b_cum_time;

        println!("Day {:02}:\n A: {} in {}\n B: {} in {}", day+1,  a_result, fmt_time(a_cum_time/runs), b_result, fmt_time(b_cum_time/runs));
    }

    println!("----------");
    println!("Cumulative time: {}", fmt_time(cum_time/runs));
}
