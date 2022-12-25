fn translate(n: usize) -> String {
    if n == 0 {
        "".to_string()
    } else {
        translate((n + 2) / 5) + ["0", "1", "2", "=", "-"][n % 5]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let fuel = input
        .lines()
        .map(|l| {
            l.chars().fold(0, |acc, b| {
                acc * 5 + "=-012".chars().position(|x| x == b).unwrap() - 2
            })
        })
        .sum::<usize>();

    dbg!(translate(fuel as usize));
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
