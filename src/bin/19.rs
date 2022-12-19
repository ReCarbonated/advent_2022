use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};

use nom::{
    bytes::complete::tag,
    bytes::complete::take_while,
    combinator::{map_res, opt},
    IResult,
};

#[derive(Debug, Copy, Clone)]
struct Blueprint {
    idx: i32,
    ore: i32,
    clay: i32,
    obsidian: (i32, i32),
    geode: (i32, i32),
}

impl Blueprint {
    fn create_ore(self, ore: i32) -> bool {
        ore >= self.ore
    }

    fn create_clay(self, ore: i32) -> bool {
        ore >= self.clay
    }

    fn create_obsidian(self, ore: i32, clay: i32) -> bool {
        ore >= self.obsidian.0 && clay >= self.obsidian.1
    }

    fn create_geode(self, ore: i32, obs: i32) -> bool {
        ore >= self.geode.0 && obs >= self.geode.1
    }

    fn get_max_cost(self) -> i32 {
        return *[self.clay, self.ore, self.obsidian.0, self.geode.0]
            .iter()
            .max()
            .unwrap();
    }
}
#[derive(Default, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct State {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
}

fn mine(mut state: State) -> State {
    state.ore += state.ore_robots;
    state.clay += state.clay_robots;
    state.obsidian += state.obsidian_robots;
    state.geode += state.geode_robots;
    state
}

fn from_int(input: &str) -> Result<usize, std::num::ParseIntError> {
    let i = usize::from_str_radix(input, 10);
    i
}

fn parse_int(input: &str) -> IResult<&str, usize> {
    let (input, _) = opt(tag("-"))(input)?;
    let x = map_res(take_while(|c: char| c.is_digit(10)), |s| from_int(s))(input);
    x
}

fn parse_input(input: &str) -> IResult<&str, Blueprint> {
    let (input, _) = tag("Blueprint ")(input)?;
    let (input, idx) = parse_int(input)?;
    let (input, _) = tag(": Each ore robot costs ")(input)?;
    let (input, ore) = parse_int(input)?;
    let (input, _) = tag(" ore. Each clay robot costs ")(input)?;
    let (input, clay) = parse_int(input)?;
    let (input, _) = tag(" ore. Each obsidian robot costs ")(input)?;
    let (input, obs1) = parse_int(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, obs2) = parse_int(input)?;
    let (input, _) = tag(" clay. Each geode robot costs ")(input)?;
    let (input, geode1) = parse_int(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, geode2) = parse_int(input)?;
    Ok((
        input,
        Blueprint {
            idx: idx as i32,
            ore: ore as i32,
            clay: clay as i32,
            obsidian: (obs1 as i32, obs2 as i32),
            geode: (geode1 as i32, geode2 as i32),
        },
    ))
}

fn max_miner(b: Blueprint, time: i32) -> i32 {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();

    //heuristic
    let max_ore_cost = b.get_max_cost(); 
    
    queue.push_back((0, {
        let mut starting_state = State::default();
        starting_state.ore_robots = 1;
        starting_state
    }));

    let mut ans = 0;

    while let Some((current_time, state)) = queue.pop_front() {
        if current_time >= time {
            ans = std::cmp::max(ans, state.geode);
            continue;
        }
        if seen.contains(&state) {
            continue;
        }

        seen.insert(state);
        // Check if can mine ore
        if state.ore_robots < max_ore_cost && b.create_ore(state.ore) {
            let mut new_state = mine(state);
            new_state.ore -= b.ore;
            new_state.ore_robots += 1;
            queue.push_back((current_time + 1, new_state));
        }
        if state.clay_robots < b.obsidian.1 && b.create_clay(state.ore) {
            let mut new_state = mine(state);
            new_state.ore -= b.clay;
            new_state.clay_robots += 1;
            queue.push_back((current_time + 1, new_state));
        }
        if state.obsidian_robots < b.geode.1 && b.create_obsidian(state.ore, state.clay) {
            let mut new_state = mine(state);
            new_state.ore -= b.obsidian.0;
            new_state.clay -= b.obsidian.1;
            new_state.obsidian_robots += 1;
            queue.push_back((current_time + 1, new_state));
        }
        if b.create_geode(state.ore, state.obsidian) {
            let mut new_state = mine(state);
            new_state.ore -= b.geode.0;
            new_state.obsidian -= b.geode.1;
            new_state.geode_robots += 1;
            queue.push_back((current_time + 1, new_state));
        } else {
            queue.push_back((current_time + 1, mine(state)));
        }
    }
    ans
}

pub fn part_one(input: &str) -> Option<u32> {
    let blueprints = input
        .lines()
        .map(|line| parse_input(line).unwrap().1)
        .collect::<Vec<Blueprint>>();

    let p1: i32 = blueprints
        .par_iter()
        .map(|b| b.idx * max_miner(*b, 24))
        .sum();
    dbg!(p1);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let blueprints = input
    .lines()
    .map(|line| parse_input(line).unwrap().1)
    .collect::<Vec<Blueprint>>();

    let p2: i32 = blueprints.par_iter().take(3).map(|&b| max_miner(b, 32)).product();
    
    dbg!(p2);
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), None);
    }
}
