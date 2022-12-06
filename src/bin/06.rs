use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut output: u32 = 0;
    let mut only_once: bool = true;
    let first_packet = &input
        .chars()
        .collect::<Vec<char>>();
    for (idx, item) in first_packet.windows(4).enumerate() {
        let test: HashSet<&char> = HashSet::from_iter(item.iter());
        if test.len() == 4 && only_once {
            output = u32::try_from(idx).unwrap() + 4;
            only_once = false;
        }
    }
    return Some(output);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut output: u32 = 0;
    let mut only_once: bool = true;
    let first_packet = &input
        .chars()
        .collect::<Vec<char>>()[..];
    for (idx, item) in first_packet.windows(14).enumerate() {
        let test: HashSet<&char> = HashSet::from_iter(item.iter());
        if test.len() == 14 && only_once {
            output = u32::try_from(idx).unwrap() + 14;
            only_once = false;
        }
    }
    return Some(output);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
