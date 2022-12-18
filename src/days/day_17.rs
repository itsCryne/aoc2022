use std::{fmt::Display, collections::HashSet};

pub fn a(input: &str) -> Box<dyn Display> {
    let mut jet = input.trim().chars().cycle();

    let mut settled_pieces = HashSet::new();
    let mut highest = -1;

    'rockloop: for i in 0_i64..2022 {
        let mut rock = match i % 5 {
            0 => (0..4).map(|x| (x, 0)).collect(),
            1 => vec![(0, 1), (1,0), (1,1), (2,1), (1,2)],
            2 => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2,2)],
            3 => (0..4).map(|y| (0, y)).collect(),
            4 => vec![(0, 0), (0,1), (1,0), (1,1)],
            _ => unreachable!()
        };

        rock.iter_mut().for_each(|(x,y)| {
            *x += 2;
            *y += highest + 4;
        });

        loop {
            match jet.next().unwrap() {
                '<' => {
                    if rock.iter().min_by_key(|(x, _)| x).unwrap().0 != 0
                        && !rock.iter().any(|(rx, ry)| settled_pieces.contains(&(rx-1, *ry))) {
                            rock.iter_mut().for_each(|(x,_)| *x -= 1);
                    }
                },
                '>' => {
                    if rock.iter().max_by_key(|(x, _)| x).unwrap().0 != 6
                        && !rock.iter().any(|(rx, ry)| settled_pieces.contains(&(rx+1, *ry))) {
                            rock.iter_mut().for_each(|(x,_)| *x += 1);
                    }
                }
                _ => unreachable!()
            }

            if rock.iter().any(|(rx, ry)| ry == &0 || settled_pieces.contains(&(*rx, ry-1))) {
                let highest_of_rock = rock.iter().max_by_key(|(_, y)| y).unwrap().1;
                if highest_of_rock > highest {
                    highest = highest_of_rock;
                }

                rock.iter().for_each(|r| { settled_pieces.insert(*r); });
                continue 'rockloop;
            } else {
                rock.iter_mut().for_each(|(_, y)| *y -= 1);
            }
        }
    }

    Box::new(highest+1)
}

pub fn b(_input: &str) -> Box<dyn Display> {
    Box::new(0)
}
