use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.lines() {
        let(first, second) = line.split_at(line.len() / 2);
        let first_set = first.chars().collect::<HashSet<char>>();
        let second_set = second.chars().collect::<HashSet<char>>();

        let intersect = first_set.intersection(&second_set).collect::<Vec<&char>>();

        
        for item in intersect  {
            let value = *item;
            let priority = match value {
                'a'..='z' => value as u8 - 'a' as u8 + 1,
                'A'..='Z' => value as u8 - 'A' as u8 + 27,
                _ => 0,
            } as u32;
            total += priority
        }
    }
    return Some(total);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.lines().collect::<Vec<_>>().chunks(3) {

        let mut sets: Vec<HashSet<char>> = Vec::new();
        sets.push(line[0].chars().collect::<HashSet<char>>());
        sets.push(line[1].chars().collect::<HashSet<char>>());
        sets.push(line[2].chars().collect::<HashSet<char>>());

        let intersection = sets
            .iter()
            .skip(1)
            .fold(sets[0].clone(), |acc, hs| {
                acc.intersection(hs).cloned().collect()
        });

        
        for item in intersection  {
            let value = item;
            let priority = match value {
                'a'..='z' => value as u8 - 'a' as u8 + 1,
                'A'..='Z' => value as u8 - 'A' as u8 + 27,
                _ => 0,
            } as u32;
            total += priority
        }
    }
    return Some(total);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}

