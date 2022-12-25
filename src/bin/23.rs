use std::{
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

fn simulation(positions: &mut HashSet<(isize, isize)>, max_iteration_count: usize) -> u32 {
    let search_order = [
        [(0, -1), (1, -1), (-1, -1)],
        [(0, 1), (1, 1), (-1, 1)],
        [(-1, 0), (-1, -1), (-1, 1)],
        [(1, 0), (1, -1), (1, 1)],
    ];

    let mut current_dir = 0;
    let mut iteration = 1;
    while iteration <= max_iteration_count {
        let mut next_positions = HashSet::new();
        let mut proposed_positions = HashMap::new();
        let mut proposed_set = HashSet::new();
        let mut conflicts = HashSet::new();

        for &(x, y) in positions.iter() {
            let move_elf = (-1..=1)
                .cartesian_product(-1..=1)
                .filter(|&(d_x, d_y)| d_x != 0 || d_y != 0)
                .any(|(d_x, d_y)| positions.contains(&(x + d_x, y + d_y)));

            let next_pos;
            if move_elf {
                let dir = search_order
                    .iter()
                    .cycle()
                    .enumerate()
                    .skip(current_dir)
                    .take(4)
                    .find(|(_, order)| {
                        order
                            .iter()
                            .all(|(d_x, d_y)| !positions.contains(&(x + d_x, y + d_y)))
                    })
                    .map(|(index, _)| index % 4);
                if let Some(dir) = dir {
                    next_pos = (x + search_order[dir][0].0, y + search_order[dir][0].1)
                } else {
                    next_pos = (x, y)
                }
            } else {
                next_pos = (x, y)
            };

            proposed_positions.insert((x, y), next_pos);
            if proposed_set.contains(&next_pos) {
                conflicts.insert(next_pos);
            }

            proposed_set.insert(next_pos);
        }

        for (from, to) in proposed_positions {
            if !conflicts.contains(&to) {
                next_positions.insert(to);
            } else {
                next_positions.insert(from);
            }
        }

        if next_positions == *positions {
            break;
        }

        *positions = next_positions;
        current_dir = (current_dir + 1) % 4;
        iteration += 1;
    }

    iteration as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut elves = input
        .lines()
        .enumerate()
        .flat_map(|(idy, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter(|(_, b)| **b == b'#')
                .map(move |(idx, _)| (idx as isize, idy as isize))
        })
        .into_iter()
        .collect::<HashSet<(isize, isize)>>();

    simulation(&mut elves, 10);

    let min_x = elves.iter().map(|(x, _)| x).min().unwrap();
    let max_x = elves.iter().map(|(x, _)| x).max().unwrap();
    let min_y = elves.iter().map(|(_, y)| y).min().unwrap();
    let max_y = elves.iter().map(|(_, y)| y).max().unwrap();

    Some(((max_x - min_x + 1) * (max_y - min_y + 1) - (elves.len() as isize)) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves = input
        .lines()
        .enumerate()
        .flat_map(|(idy, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter(|(_, b)| **b == b'#')
                .map(move |(idx, _)| (idx as isize, idy as isize))
        })
        .into_iter()
        .collect::<HashSet<(isize, isize)>>();

    Some(simulation(&mut elves, usize::MAX))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), None);
    }
}
