use std::{fmt::Display, collections::{HashMap, HashSet}};

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

    let min_x = locations.iter().fold(i64::MAX, |acc, cur| {
        if cur.0.0 - (*cur.1 as i64) < acc {
            cur.0.0 - (*cur.1 as i64)
        } else {
            acc
        }
    });

    let max_x = locations.iter().fold(i64::MIN, |acc, cur| {
        if cur.0.0 + (*cur.1 as i64) > acc {
            cur.0.0 + (*cur.1 as i64)
        } else {
            acc
        }
    });


    let y = 2000000;
    let mut counter = 0;
    for x in min_x..=max_x {
        if close_to_sensor((x, y), &locations) && !beacons.contains(&(x,y)) {
            counter += 1;
        }
    }

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

    let search_space_size = 4000000;
    for (s, d) in &locations {
        let mut outside = HashSet::new();

        outside.insert((s.0-(d+1), s.1));
        outside.insert((s.0+(d+1), s.1));

        for x in (s.0-*d)..=(s.0+*d) {
            let p1 = (x, (s.1 - (*d - (x.abs_diff(s.0) as i64)))-1);
            let p2 = (x, s.1 + (*d - (x.abs_diff(s.0) as i64))+1);

            outside.insert(p1);
            outside.insert(p2);
        }

        for (x,y) in outside {
            if !(0..=search_space_size).contains(&x) || !(0..=search_space_size).contains(&y) {
                continue;
            }
            if !close_to_sensor((x, y), &locations) && !beacons.contains(&(x,y)) {
                return Box::new(x * 4000000 + y);
            }
        }
    }
    unreachable!()
}
