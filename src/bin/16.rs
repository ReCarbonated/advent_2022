use std::{
    collections::{HashMap},
};
use ndarray::Array3;

use nom::{
    branch::alt,
    bytes::complete::take_while,
    bytes::complete::{is_not, tag},
    combinator::{map_res, opt},
    IResult,
};

fn from_int(input: &str) -> Result<usize, std::num::ParseIntError> {
    let i = usize::from_str_radix(input, 10);
    i
}

fn parse_int(input: &str) -> IResult<&str, usize> {
    let (input, _) = opt(tag("-"))(input)?;
    let x = map_res(take_while(|c: char| c.is_digit(10)), |s| {
        from_int(s)
    })(input);
    x
}

struct Pipe<'a> {
    Valve: &'a str,
    Flow: u32,
    Paths: Vec<&'a str>,
}

fn parse_input(input: &str) -> IResult<&str, Pipe> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, valve) = is_not(" ")(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, rate) = parse_int(input)?;
    let (input, _) = alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
    ))(input)?;
    dbg!(input);
    Ok((
        input,
        Pipe {
            Valve: valve,
            Flow: rate as u32,
            Paths: input.split(", ").collect::<Vec<_>>(),
        },
    ))
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut pipes = input
        .lines()
        .map(|line| parse_input(line).unwrap().1)
        .collect::<Vec<Pipe>>();

    pipes.sort_by(|a, b| b.Flow.cmp(&a.Flow));

    let lookup = pipes
        .iter()
        .enumerate()
        .map(|(i, v)| (v.Valve, i))
        .collect::<HashMap<_, _>>();

    let m = pipes.iter().filter(|p| p.Flow > 0).count();
    let n = pipes.len();

    let mut obj = vec![vec![0usize; 0]; n];
    let mut flow = vec![0u32; n];

    for v in pipes.iter() {
        let i = lookup[v.Valve];
        flow[i] = v.Flow;
        for w in v.Paths.iter() {
            obj[i].push(lookup[*w]);
        }
    }
    let aa = lookup["AA"];

    let memo = 1 << m;
    let mut cache = Array3::<u32>::zeros([30, n, memo]);

    for t in 1..30 {
        for i in 0..n {
            let ii = 1 << i;
            for x in 0..memo {
                let mut o = cache[(t, i, x)];
                if ii & x != 0 && t >= 2 {
                    o = o.max(cache[(t - 1, i, x - ii)] + flow[i] * t as u32);
                }
                for &j in obj[i].iter() {
                    o = o.max(cache[(t - 1, j, x)]);
                }
                cache[(t, i, x)] = o;
            }
        }
    }
    let res = cache[(29, aa, memo - 1)];

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut pipes = input
        .lines()
        .map(|line| parse_input(line).unwrap().1)
        .collect::<Vec<Pipe>>();

    pipes.sort_by(|a, b| b.Flow.cmp(&a.Flow));

    let lookup = pipes
        .iter()
        .enumerate()
        .map(|(i, v)| (v.Valve, i))
        .collect::<HashMap<_, _>>();

    let m = pipes.iter().filter(|p| p.Flow > 0).count();
    let n = pipes.len();

    let mut obj = vec![vec![0usize; 0]; n];
    let mut flow = vec![0u32; n];

    for v in pipes.iter() {
        let i = lookup[v.Valve];
        flow[i] = v.Flow;
        for w in v.Paths.iter() {
            obj[i].push(lookup[*w]);
        }
    }
    let aa = lookup["AA"];

    let memo = 1 << m;
    let mut cache = Array3::<u32>::zeros([30, n, memo]);

    for t in 1..30 {
        for i in 0..n {
            let ii = 1 << i;
            for x in 0..memo {
                let mut o = cache[(t, i, x)];
                if ii & x != 0 && t >= 2 {
                    o = o.max(cache[(t - 1, i, x - ii)] + flow[i] * t as u32);
                }
                for &j in obj[i].iter() {
                    o = o.max(cache[(t - 1, j, x)]);
                }
                cache[(t, i, x)] = o;
            }
        }
    }

    let mut best = 0;
    for x in 0..memo / 2 {
        let y = memo - 1 - x;
        best = best.max(cache[(25, aa, x)] + cache[(25, aa, y)]);
    }
    Some(best)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
