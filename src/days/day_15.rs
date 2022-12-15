use std::{fmt::Display, collections::{HashMap, HashSet}};

use rayon::prelude::{IntoParallelIterator, ParallelIterator, IntoParallelRefIterator};

fn manhattan_distance(l1: (i64, i64), l2: (i64, i64)) -> i64 {
    (l1.0.abs_diff(l2.0) + l1.1.abs_diff(l2.1)) as i64
}

fn close_to_sensor(p: (i64, i64), l: &HashMap<(i64, i64), i64>) -> bool {
    return l.iter().any(|(s, d)| manhattan_distance(*s, p) <= *d);
}

pub fn a(input: &str) -> Box<dyn Display> {
    let mut locations = HashMap::new();
    let mut beacons = HashSet::new();

    for line in input.lines() {
        let words = line.split(" ").collect::<Vec<&str>>();

        let sx = i64::from_str_radix(&words[2].replace("x=", "").replace(",", ""), 10).unwrap();
        let sy = i64::from_str_radix(&words[3].replace("y=", "").replace(":", ""), 10).unwrap();
        let bx = i64::from_str_radix(&words[8].replace("x=", "").replace(",", ""), 10).unwrap();
        let by = i64::from_str_radix(&words[9].replace("y=", ""), 10).unwrap();

        beacons.insert((bx, by));
        locations.insert((sx, sy), manhattan_distance((sx, sy), (bx, by)));
    }

    let min_x = locations.par_iter().fold(|| i64::MAX, |acc, cur| {
        if cur.0.0 - (*cur.1 as i64) < acc {
            cur.0.0 - (*cur.1 as i64)
        } else {
            acc
        }
    }).min().unwrap();

    let max_x = locations.par_iter().fold(|| i64::MIN, |acc, cur| {
        if cur.0.0 + (*cur.1 as i64) > acc {
            cur.0.0 + (*cur.1 as i64)
        } else {
            acc
        }
    }).max().unwrap();

    let y = 2000000;
    let counter = (min_x..max_x).into_par_iter().filter_map(|x| {
        if close_to_sensor((x, y), &locations) && !beacons.contains(&(x,y)) {
            Some(1)
        } else {
            None
        }
    }).count();

    /*
    for x in min_x..=max_x {
        if close_to_sensor((x, y), &locations) && !beacons.contains(&(x,y)) {
            counter += 1;
        }
    }
    */

    Box::new(counter)
}

pub fn b(input: &str) -> Box<dyn Display> {
    let mut locations = HashMap::new();
    let mut beacons = HashSet::new();

    for line in input.lines() {
        let words = line.split(" ").collect::<Vec<&str>>();

        let sx = i64::from_str_radix(&words[2].replace("x=", "").replace(",", ""), 10).unwrap();
        let sy = i64::from_str_radix(&words[3].replace("y=", "").replace(":", ""), 10).unwrap();
        let bx = i64::from_str_radix(&words[8].replace("x=", "").replace(",", ""), 10).unwrap();
        let by = i64::from_str_radix(&words[9].replace("y=", ""), 10).unwrap();

        beacons.insert((bx, by));
        locations.insert((sx, sy), manhattan_distance((sx, sy), (bx, by)));
    }
    
    let pos = locations.par_iter().find_map_any(|(s, d)| {
        ((s.0-*d)..=(s.0+*d)).into_par_iter().flat_map(|x| {
            let p1 = (x, (s.1 - (*d - (x.abs_diff(s.0) as i64)))-1);
            let p2 = (x, s.1 + (*d - (x.abs_diff(s.0) as i64))+1);

            [p1, p2]
        }).find_map_any(|(x,y)| {
            if !close_to_sensor((x, y), &locations) && !beacons.contains(&(x, y)) {
                Some((x, y))
            } else {
                None
            }
        })
    }).unwrap();

    Box::new(pos.0 * 4000000 + pos.1)
}
