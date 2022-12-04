pub fn part_one(input: &str) -> Option<u32> {
    let output = input
        .lines()
        .map(|v| v.parse::<u32>().ok())
        .collect::<Vec<_>>()
        .split(|s| s.is_none())
        .map(|g| g.iter().map(|v| v.unwrap()).sum::<u32>())
        .max();
    return output;
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut output = input
        .lines()
        .map(|v| v.parse::<u32>().ok())
        .collect::<Vec<_>>()
        .split(|s| s.is_none())
        .map(|g| g.iter().map(|v| v.unwrap()).sum::<u32>())
        .collect::<Vec<_>>();
    output.sort();

    let final_length = output.split_off(output.len().saturating_sub(3));
    let part_two = final_length
        .iter()
        .sum::<u32>();
    return Some(part_two);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
