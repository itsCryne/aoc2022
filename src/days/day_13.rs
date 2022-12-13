use std::fmt::Display;

use nom::{IResult, character::complete::char, combinator::recognize, multi::{many1, many0, separated_list0}, sequence::{terminated, preceded}, character::complete::one_of, bytes::complete::tag, branch::alt};

#[derive(Debug,Clone, PartialEq, Eq)]
enum Packet {
    Data(usize),
    Subpacket(Vec<Packet>)
}

//Thank you, Luke
fn data_parser(input: &str) -> IResult<&str, Packet> {
    let (rest, num_str) =
        recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)?;
    let num = num_str.parse::<usize>().unwrap();
    IResult::Ok((rest, Packet::Data(num)))
}

fn subpacket_parser(input: &str) -> IResult<&str, Packet> {
    let (rest, v) = separated_list0(tag(","), packet_parser)(input)?;
    IResult::Ok((rest, Packet::Subpacket(v)))
}

fn packet_parser(input: &str) -> IResult<&str, Packet> {
    alt((
        data_parser,
        terminated(preceded(char('['), subpacket_parser), char(']')),
    ))(input)
}

fn compare_packets_recursively(p1: Vec<Packet>, p2: Vec<Packet>) -> Option<bool> {
    let ret_after_loop = if let (Some(Packet::Subpacket(sp1)), Some(Packet::Subpacket(sp2))) = (p1.get(0).clone(), p2.get(0).clone()) {
        match sp1.len().cmp(&sp2.len()) {
            std::cmp::Ordering::Less => Some(true),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => Some(false),
        }
    } else {
        match p1.len().cmp(&p2.len()) {
            std::cmp::Ordering::Less => Some(true),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => Some(false),
        }
    };

    for sub in p1.into_iter().zip(p2) {
        match sub {
            (Packet::Data(d1), Packet::Data(d2)) =>  {
                match d1.cmp(&d2) {
                    std::cmp::Ordering::Less => return Some(true),
                    std::cmp::Ordering::Equal => continue,
                    std::cmp::Ordering::Greater => return Some(false),
                }
            },
            (Packet::Data(d), Packet::Subpacket(s)) => {
                return compare_packets_recursively(vec![Packet::Subpacket(vec![Packet::Data(d)])], vec![Packet::Subpacket(s)]);
            },
            (Packet::Subpacket(s), Packet::Data(d)) => {
                return compare_packets_recursively(vec![Packet::Subpacket(s)], vec![Packet::Subpacket(vec![Packet::Data(d)])]);
            },
            (Packet::Subpacket(s1), Packet::Subpacket(s2)) => {
                if let Some(result) = compare_packets_recursively(s1, s2) {
                    return Some(result);
                } else {
                    continue;
                }
            },
        }
    }

    ret_after_loop
}

pub fn a(input: &str) -> Box<dyn Display> {
    let mut counter = 0;

    for (idx, packetpair) in input.split("\n\n").enumerate() {
        let (ip1, ip2) = packetpair.split_once("\n").unwrap();
        let p1 = packet_parser(ip1).unwrap().1;
        let p2 = packet_parser(ip2).unwrap().1;

        if compare_packets_recursively(vec![p1], vec![p2]).unwrap() {
            counter += idx+1;
        }
    }

    Box::new(counter)
}


pub fn b(input: &str) -> Box<dyn Display> {
    let mut packets = Vec::new();

    let divider_2 = Packet::Subpacket(vec![Packet::Subpacket(vec![Packet::Data(2)])]);
    let divider_6 = Packet::Subpacket(vec![Packet::Subpacket(vec![Packet::Data(6)])]);

    packets.push(divider_2.clone());
    packets.push(divider_6.clone());

    for line in input.lines() {
        if line == "" {
            continue;
        } else {
            packets.push(packet_parser(line).unwrap().1);
        }
    }

    packets.sort_by(|a, b| {
        match compare_packets_recursively(vec![a.clone()], vec![b.clone()]) {
            Some(correct) => {
                if correct {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            },
            None => {
                std::cmp::Ordering::Equal
            },
        }
    });

    let divider_2_position = packets.iter().position(|p| p == &divider_2).unwrap() + 1;
    let divider_6_position = packets.iter().position(|p| p == &divider_6).unwrap() + 1;

    Box::new(divider_6_position * divider_2_position)
}
