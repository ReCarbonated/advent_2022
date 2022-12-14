// use core::num::dec2flt::parse;
// use std::collections::VecDeque;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::i64, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum Packet {
    Int(i64),
    List(Vec<Packet>),
}

impl std::cmp::PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl Eq for Packet {}
impl std::cmp::Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::cmp::PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Int(l), Packet::Int(r)) => l.partial_cmp(r),
            (Packet::List(_), Packet::Int(_)) => {
                self.partial_cmp(&Packet::List(vec![other.clone()]))
            }
            (Packet::Int(_), Packet::List(_)) => {
                Packet::List(vec![self.clone()]).partial_cmp(other)
            }
            (Packet::List(l), Packet::List(r)) => {
                for (val1, val2) in l.iter().zip(r) {
                    if let Some(result) = val1.partial_cmp(val2) {
                        if result != Ordering::Equal {
                            return Some(result);
                        }
                    }
                }
                l.len().partial_cmp(&r.len())
            }
        }
    }
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        map(i64, |v| Packet::Int(v)),
        map(
            delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]")),
            |v| Packet::List(v),
        ),
    ))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut count: u32 = 0;
    for (i, packet_group) in input.split("\n\n").enumerate() {
        let mut lines = packet_group.lines();
        let left = parse_packet(lines.next().unwrap()).unwrap().1;
        let right = parse_packet(lines.next().unwrap()).unwrap().1;

        if left < right {
            count += i as u32 + 1 as u32;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut packets: Vec<Packet> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| parse_packet(l).unwrap().1)
        .collect();
    let div1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort();
    let pos1 = packets.iter().position(|p| p == &div1).unwrap() + 1;
    let pos2 = packets.iter().position(|p| p == &div2).unwrap() + 1;

    return Some((pos1 * pos2) as u32);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
