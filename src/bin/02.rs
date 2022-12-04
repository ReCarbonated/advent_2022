pub fn part_one(input: &str) -> Option<u32> {
    let output = input
        .lines()
        .map(|round| {
            match round {
                "A X" => 4,
                "A Y" => 8,
                "A Z" => 3,
                "B X" => 1,
                "B Y" => 5,
                "B Z" => 9,
                "C X" => 7,
                "C Y" => 2,
                "C Z" => 6,
                _ => 0
            }
        })
        .sum::<u32>();
    return Some(output);
}

pub fn part_two(input: &str) -> Option<u32> {
    let output = input
        .lines()
        .map(|round| {
            match round {
                "A X" => 2,
                "B X" => 3,
                "C X" => 1,
                "A Y" => 4,
                "B Y" => 5,
                "C Y" => 6,
                "A Z" => 9,
                "B Z" => 7,
                "C Z" => 8,
                _ => 0
            }
        })
        .sum::<u32>();
    return Some(output);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
