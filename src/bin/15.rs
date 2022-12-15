use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take_while,
    character::complete::i64,
    combinator::{map, map_res, opt},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn distance(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn from_int(input: &str, neg: bool) -> Result<isize, std::num::ParseIntError> {
    let i = isize::from_str_radix(input, 10);
    i.map(|i| if neg { -i } else { i })
}

fn parse_int(input: &str) -> IResult<&str, isize> {
    let (input, neg) = opt(tag("-"))(input)?;
    let x = map_res(take_while(|c: char| c.is_digit(10)), |s| {
        from_int(s, neg.is_some())
    })(input);
    x
}

fn parse_input(input: &str) -> IResult<&str, ((isize, isize), (isize, isize))> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, x1) = parse_int(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y1) = parse_int(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, x2) = parse_int(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y2) = parse_int(input)?;
    Ok((input, ((x1, y1), (x2, y2))))
}

pub fn part_one(input: &str) -> Option<u32> {
    let groups = input
        .lines()
        .map(|l| parse_input(&l).unwrap().1)
        .map(|(a, b)| {
            (
                Point {
                    x: a.0 as i64,
                    y: a.1 as i64,
                },
                Point {
                    x: b.0 as i64,
                    y: b.1 as i64,
                },
            )
        })
        .collect::<Vec<_>>();

    let mut cover = HashSet::new();
    let mut beacons = HashSet::new();

    for (s, b) in groups {
        if b.y == 2000000 {
            beacons.insert(b.x);
        }

        let r = s.distance(&b) as i64;
        let rx = r - s.y.abs_diff(2000000) as i64;

        if rx < 0 {
            continue;
        }
        let x_first = s.x - rx;
        let x_last = s.x + rx;

        for x in x_first..=x_last {
            cover.insert(x);
        }

        let output = cover.difference(&beacons).collect_vec().len();
        println!("{:?}", output);
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let groups = input
        .lines()
        .map(|l| parse_input(&l).unwrap().1)
        .map(|(a, b)| {
            (
                Point {
                    x: a.0 as i64,
                    y: a.1 as i64,
                },
                Point {
                    x: b.0 as i64,
                    y: b.1 as i64,
                },
            )
        })
        .collect::<Vec<_>>();


    let mut x = 0;
    let mut y = 0;

    let val = 4000000;

    'outer: while y <= val {
        if val < x {
            x = 0;
            y += 1;
        }
        for (s, b) in groups.iter() {
            let r = s.distance(&b) as i64;
            let rx = r - s.y.abs_diff(y) as i64;
            if rx < 0 {
                continue;
            }

            let x_first = s.x - rx;
            let x_last = s.x + rx;
            if x_first <= x && x <= x_last {
                x = x_last + 1;
                continue 'outer;
            }
        }
        println!("{}", x * val + y);
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    // advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
